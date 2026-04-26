use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use fileflow_core::{Action, Context, FileFlowError, Progress, Result};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub delete_extra: bool,
    pub overwrite: bool,
    pub recursive: bool,
}

pub struct SyncAction {
    cfg: SyncConfig,
}

impl SyncAction {
    pub fn new(cfg: SyncConfig) -> Self {
        Self { cfg }
    }
}

impl Action for SyncAction {
    fn name(&self) -> &'static str {
        "sync"
    }

    fn execute(&self, ctx: &mut Context) -> Result<()> {
        ctx.ensure_not_cancelled()?;

        let src = &self.cfg.src;
        let dst = &self.cfg.dst;

        validate_source_dir(src)?;
        fs::create_dir_all(dst)?;

        let src_files = read_files(src, self.cfg.recursive)?;
        let dst_files = read_files(dst, self.cfg.recursive)?;

        let total_steps =
            src_files.len() as u64 + if self.cfg.delete_extra { dst_files.len() as u64 } else { 0 };

        ctx.info(format!(
            "SyncAction: {} -> {} | recursive={} | source files={}",
            src.display(),
            dst.display(),
            self.cfg.recursive,
            src_files.len()
        ));

        ctx.set_progress(Progress::new(0, total_steps).with_message("Iniciando sync"));

        let mut processed = 0u64;

        let dst_map = build_relative_map(dst, &dst_files)?;
        let mut src_rel_paths = HashSet::new();

        for src_file in &src_files {
            ctx.ensure_not_cancelled()?;

            let rel = relative_key(src, src_file)?;
            src_rel_paths.insert(rel.clone());

            let dst_file = dst.join(&rel);

            if let Some(parent) = dst_file.parent() {
                fs::create_dir_all(parent)?;
            }

            if let Some(existing_dst) = dst_map.get(&rel) {
                if self.cfg.overwrite || should_copy(src_file, existing_dst)? {
                    ctx.info(format!("SyncAction: updating {}", rel.display()));
                    fs::copy(src_file, &dst_file)?;
                } else {
                    ctx.info(format!("SyncAction: skipping {}", rel.display()));
                }
            } else {
                ctx.info(format!("SyncAction: copying {}", rel.display()));
                fs::copy(src_file, &dst_file)?;
            }

            processed += 1;
            ctx.set_progress(
                Progress::new(processed, total_steps)
                    .with_message(format!("Procesado: {}", rel.display())),
            );
        }

        if self.cfg.delete_extra {
            for dst_file in &dst_files {
                ctx.ensure_not_cancelled()?;

                let rel = relative_key(dst, dst_file)?;

                if !src_rel_paths.contains(&rel) {
                    ctx.info(format!("SyncAction: deleting extra {}", rel.display()));
                    fs::remove_file(dst_file)?;
                }

                processed += 1;
                ctx.set_progress(
                    Progress::new(processed, total_steps)
                        .with_message(format!("Revisado destino: {}", rel.display())),
                );
            }
        }

        ctx.info("SyncAction: OK");
        Ok(())
    }
}

fn validate_source_dir(src: &Path) -> Result<()> {
    if !src.exists() {
        return Err(FileFlowError::Message(format!(
            "Source directory does not exist: {}",
            src.display()
        )));
    }

    if !src.is_dir() {
        return Err(FileFlowError::Message(format!(
            "Source is not a directory: {}",
            src.display()
        )));
    }

    Ok(())
}

fn read_files(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if recursive {
        for entry in WalkDir::new(dir) {
            let entry = entry.map_err(|e| FileFlowError::Message(e.to_string()))?;
            let path = entry.path();

            if path.is_file() {
                files.push(path.to_path_buf());
            }
        }
    } else {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                files.push(path);
            }
        }
    }

    files.sort();
    Ok(files)
}

fn build_relative_map(base: &Path, files: &[PathBuf]) -> Result<HashMap<PathBuf, PathBuf>> {
    let mut map = HashMap::new();

    for file in files {
        let rel = relative_key(base, file)?;
        map.insert(rel, file.clone());
    }

    Ok(map)
}

fn relative_key(base: &Path, file: &Path) -> Result<PathBuf> {
    file.strip_prefix(base)
        .map(|p| p.to_path_buf())
        .map_err(|e| {
            FileFlowError::Message(format!(
                "Could not calculate relative path for '{}': {}",
                file.display(),
                e
            ))
        })
}

fn should_copy(src: &Path, dst: &Path) -> Result<bool> {
    let src_meta = fs::metadata(src)?;
    let dst_meta = fs::metadata(dst)?;

    if src_meta.len() != dst_meta.len() {
        return Ok(true);
    }

    let src_modified = modified_or_epoch(&src_meta);
    let dst_modified = modified_or_epoch(&dst_meta);

    Ok(src_modified > dst_modified)
}

fn modified_or_epoch(meta: &fs::Metadata) -> SystemTime {
    meta.modified().unwrap_or(SystemTime::UNIX_EPOCH)
}