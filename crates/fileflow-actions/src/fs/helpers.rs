use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use fileflow_core::{Context, FileFlowError, Progress, Result};

pub const LARGE_FILE_THRESHOLD_BYTES: u64 = 10 * 1024 * 1024 * 1024; // 10 GB
const COPY_BUFFER_SIZE: usize = 8 * 1024 * 1024; // 8 MB
const PROGRESS_EMIT_EVERY_BYTES: u64 = 64 * 1024 * 1024; // 64 MB

pub fn validate_source_file(src: &Path) -> Result<()> {
    if !src.exists() {
        return Err(FileFlowError::Message(format!(
            "Source does not exist: {}",
            src.display()
        )));
    }

    if !src.is_file() {
        return Err(FileFlowError::Message(format!(
            "Source is not a file: {}",
            src.display()
        )));
    }

    Ok(())
}

pub fn ensure_distinct_paths(src: &Path, dst: &Path, operation: &str) -> Result<()> {
    let src_path = fs::canonicalize(src)?;
    let dst_path = comparable_destination_path(dst)?;

    if paths_equal(&src_path, &dst_path) {
        return Err(FileFlowError::Message(format!(
            "Cannot {operation} a file onto itself: {}",
            src.display()
        )));
    }

    Ok(())
}

pub fn prepare_destination(dst: &Path, overwrite: bool) -> Result<()> {
    if dst.exists() && !overwrite {
        return Err(FileFlowError::Message(format!(
            "Destination exists (use --overwrite): {}",
            dst.display()
        )));
    }

    if let Some(parent) = dst.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(())
}

pub fn remove_destination_if_overwrite(dst: &Path, overwrite: bool) -> Result<()> {
    if dst.exists() && overwrite {
        if dst.is_file() {
            fs::remove_file(dst)?;
        } else {
            return Err(FileFlowError::Message(format!(
                "Destination exists and is not a file: {}",
                dst.display()
            )));
        }
    }

    Ok(())
}

pub fn copy_file_optimized(
    src: &Path,
    dst: &Path,
    ctx: &mut Context,
    progress_offset: u64,
    total_progress: u64,
    label: impl Into<String>,
) -> Result<u64> {
    let label = label.into();

    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }

    let size = fs::metadata(src)?.len();
    let tmp = temp_path_for(dst);

    if tmp.exists() {
        fs::remove_file(&tmp)?;
    }

    if size >= LARGE_FILE_THRESHOLD_BYTES {
        ctx.info(format!(
            "FileFlow: archivo grande detectado ({} GB). Usando copia segura con temporal.",
            size / 1024 / 1024 / 1024
        ));
    } else {
        ctx.info("FileFlow: usando copia segura con archivo temporal");
    }

    match copy_file_chunked(src, &tmp, ctx, progress_offset, total_progress, label) {
        Ok(_) => {
            if dst.exists() {
                if dst.is_file() {
                    fs::remove_file(dst)?;
                } else {
                    let _ = fs::remove_file(&tmp);
                    return Err(FileFlowError::Message(format!(
                        "Destination exists and is not a file: {}",
                        dst.display()
                    )));
                }
            }

            fs::rename(&tmp, dst).map_err(|e| {
                let _ = fs::remove_file(&tmp);
                FileFlowError::Message(format!(
                    "Could not replace '{}' after copying '{}': {}",
                    dst.display(),
                    src.display(),
                    e
                ))
            })?;

            Ok(size)
        }
        Err(e) => {
            let _ = fs::remove_file(&tmp);
            Err(e)
        }
    }
}

fn copy_file_chunked(
    src: &Path,
    dst: &Path,
    ctx: &mut Context,
    progress_offset: u64,
    total_progress: u64,
    label: String,
) -> Result<()> {
    let total_size = fs::metadata(src)?.len();

    let input = File::open(src)?;
    let output = File::create(dst)?;

    let mut reader = BufReader::with_capacity(COPY_BUFFER_SIZE, input);
    let mut writer = BufWriter::with_capacity(COPY_BUFFER_SIZE, output);

    let mut buffer = vec![0u8; COPY_BUFFER_SIZE];
    let mut copied: u64 = 0;
    let mut last_emit: u64 = 0;

    ctx.set_progress(
        Progress::new(progress_offset, total_progress.max(1))
            .with_message(format!("Iniciando: {}", label)),
    );

    loop {
        ctx.ensure_not_cancelled()?;

        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }

        writer.write_all(&buffer[..read])?;
        copied += read as u64;

        if copied - last_emit >= PROGRESS_EMIT_EVERY_BYTES || copied == total_size {
            last_emit = copied;

            ctx.set_progress(
                Progress::new(progress_offset + copied, total_progress.max(1))
                    .with_message(label.clone()),
            );
        }
    }

    writer.flush()?;

    ctx.set_progress(
        Progress::new(progress_offset + total_size, total_progress.max(1))
            .with_message(format!("Completado: {}", label)),
    );

    Ok(())
}

fn temp_path_for(dst: &Path) -> PathBuf {
    let file_name = dst
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("fileflow-file");

    let pid = std::process::id();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);

    dst.with_file_name(format!("{file_name}.fileflow.{pid}.{nanos}.tmp"))
}

fn comparable_destination_path(dst: &Path) -> Result<PathBuf> {
    if dst.exists() {
        return Ok(fs::canonicalize(dst)?);
    }

    normalize_path(dst)
}

pub fn normalize_path(path: &Path) -> Result<PathBuf> {
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };

    let mut normalized = PathBuf::new();

    for component in joined.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
        }
    }

    Ok(normalized)
}

pub fn paths_equal(left: &Path, right: &Path) -> bool {
    #[cfg(windows)]
    {
        comparable_path_string(left) == comparable_path_string(right)
    }

    #[cfg(not(windows))]
    {
        left == right
    }
}

pub fn path_is_within(child: &Path, parent: &Path) -> bool {
    if paths_equal(child, parent) {
        return true;
    }

    #[cfg(windows)]
    {
        let child = comparable_path_string(child);
        let parent = comparable_path_string(parent);
        child.starts_with(&format!("{parent}\\"))
    }

    #[cfg(not(windows))]
    {
        child.starts_with(parent)
    }
}

#[cfg(windows)]
fn comparable_path_string(path: &Path) -> String {
    let mut path = path.to_string_lossy().replace('/', "\\");

    if let Some(stripped) = path.strip_prefix("\\\\?\\") {
        path = stripped.to_string();
    }

    path.trim_end_matches('\\').to_ascii_lowercase()
}
