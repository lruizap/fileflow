use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use fileflow_core::{Action, Context, FileFlowError, Progress, Result};

#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub delete_extra: bool,
    pub overwrite: bool,
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

        let src_files = read_top_level_files(src)?;
        let dst_files = read_top_level_files(dst)?;

        let total_steps = src_files.len() as u64 + if self.cfg.delete_extra { dst_files.len() as u64 } else { 0 };
        ctx.info(format!(
            "SyncAction: {} -> {} ({} source files)",
            src.display(),
            dst.display(),
            src_files.len()
        ));
        ctx.set_progress(Progress::new(0, total_steps).with_message("Iniciando sync"));

        let mut processed = 0u64;

        let dst_map: HashMap<String, PathBuf> = dst_files
            .iter()
            .map(|p| (file_name_string(p).unwrap_or_default(), p.clone()))
            .collect();

        let mut src_names = HashSet::new();

        for src_file in &src_files {
            ctx.ensure_not_cancelled()?;

            let file_name = file_name_string(src_file)?;
            src_names.insert(file_name.clone());

            let dst_file = dst.join(&file_name);

            if let Some(existing_dst) = dst_map.get(&file_name) {
                if should_copy(src_file, existing_dst)? {
                    ctx.info(format!("SyncAction: updating {}", file_name));
                    fs::copy(src_file, &dst_file)?;
                } else {
                    ctx.info(format!("SyncAction: skipping {}", file_name));
                }
            } else {
                ctx.info(format!("SyncAction: copying {}", file_name));
                fs::copy(src_file, &dst_file)?;
            }

            processed += 1;
            ctx.set_progress(
                Progress::new(processed, total_steps)
                    .with_message(format!("Procesado: {}", file_name)),
            );
        }

        if self.cfg.delete_extra {
            for dst_file in &dst_files {
                ctx.ensure_not_cancelled()?;

                let file_name = file_name_string(dst_file)?;
                if !src_names.contains(&file_name) {
                    ctx.info(format!("SyncAction: deleting extra {}", file_name));
                    fs::remove_file(dst_file)?;
                }

                processed += 1;
                ctx.set_progress(
                    Progress::new(processed, total_steps)
                        .with_message(format!("Revisado destino: {}", file_name)),
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

fn read_top_level_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    files.sort();
    Ok(files)
}

fn file_name_string(path: &Path) -> Result<String> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            FileFlowError::Message(format!(
                "Invalid file name for path: {}",
                path.display()
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