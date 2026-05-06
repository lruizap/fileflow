use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

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

    if size == 0 {
        File::create(dst)?;
        ctx.set_progress(
            Progress::new(progress_offset, total_progress.max(1))
                .with_message(format!("{} (archivo vacío)", label)),
        );
        return Ok(0);
    }

    if size >= LARGE_FILE_THRESHOLD_BYTES {
        ctx.info(format!(
            "FileFlow: archivo grande detectado ({} GB). Usando copia directa optimizada.",
            size / 1024 / 1024 / 1024
        ));

        match copy_file_chunked(src, dst, ctx, progress_offset, total_progress, label) {
            Ok(_) => Ok(size),
            Err(e) => {
                if matches!(e, FileFlowError::Cancelled) {
                    let _ = fs::remove_file(dst);
                }
                Err(e)
            }
        }
    } else {
        ctx.info("FileFlow: usando copia segura con archivo temporal");

        let tmp = temp_path_for(dst);

        if tmp.exists() {
            fs::remove_file(&tmp)?;
        }

        match copy_file_chunked(src, &tmp, ctx, progress_offset, total_progress, label) {
            Ok(_) => {
                if dst.exists() {
                    fs::remove_file(dst)?;
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

    dst.with_file_name(format!("{file_name}.fileflow.tmp"))
}