use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use fileflow_core::{Action, Context, FileFlowError, Progress, Result};
use walkdir::WalkDir;

use crate::fs::helpers::{copy_file_optimized, normalize_path, path_is_within, paths_equal};

#[derive(Debug, Clone)]
pub struct SyncConfig {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub delete_extra: bool,
    pub overwrite: bool,
    pub recursive: bool,
    pub dry_run: bool,
}

pub struct SyncAction {
    cfg: SyncConfig,
}

#[derive(Debug, Clone)]
struct FileEntry {
    path: PathBuf,
    rel: PathBuf,
    size: u64,
    modified: SystemTime,
}

#[derive(Debug, Default)]
struct SyncSummary {
    copied: u64,
    updated: u64,
    skipped: u64,
    deleted: u64,
    bytes_planned: u64,
    bytes_processed: u64,
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

        validate_sync_paths(src, dst)?;
        let src_base = fs::canonicalize(src)?;
        let dst_base = if self.cfg.dry_run && !dst.exists() {
            normalize_path(dst)?
        } else {
            if !self.cfg.dry_run {
                fs::create_dir_all(dst)?;
            }

            fs::canonicalize(dst)?
        };

        let src_files = read_files(&src_base, self.cfg.recursive)?;
        let dst_files = if self.cfg.dry_run && !dst.exists() {
            Vec::new()
        } else {
            read_files(&dst_base, self.cfg.recursive)?
        };

        let total_bytes = calculate_total_bytes(&src_files)?.max(1);

        ctx.info(format!(
            "SyncAction: {} -> {} | recursive={} | dry_run={} | source files={}",
            src_base.display(),
            dst_base.display(),
            self.cfg.recursive,
            self.cfg.dry_run,
            src_files.len()
        ));

        ctx.set_progress(
            Progress::new(0, total_bytes).with_message(if self.cfg.dry_run {
                "Analizando sync"
            } else {
                "Iniciando sync"
            }),
        );

        let dst_map = build_relative_map(&dst_files);
        let mut src_rel_paths = HashSet::new();
        let mut processed_bytes = 0u64;
        let mut summary = SyncSummary::default();

        for src_file in &src_files {
            ctx.ensure_not_cancelled()?;

            src_rel_paths.insert(src_file.rel.clone());

            let dst_file = dst_base.join(&src_file.rel);

            if let Some(existing_dst) = dst_map.get(&src_file.rel) {
                if self.cfg.overwrite || should_copy(src_file, existing_dst) {
                    summary.updated += 1;
                    summary.bytes_planned += src_file.size;
                    ctx.info(format!("SyncAction: updating {}", src_file.rel.display()));

                    if !self.cfg.dry_run {
                        copy_file_optimized(
                            &src_file.path,
                            &dst_file,
                            ctx,
                            processed_bytes,
                            total_bytes,
                            src_file.rel.display().to_string(),
                        )?;
                    }
                } else {
                    summary.skipped += 1;
                    ctx.info(format!("SyncAction: skipping {}", src_file.rel.display()));
                    ctx.set_progress(
                        Progress::new(processed_bytes + src_file.size, total_bytes)
                            .with_message(format!("Saltado: {}", src_file.rel.display())),
                    );
                }
            } else {
                summary.copied += 1;
                summary.bytes_planned += src_file.size;
                ctx.info(format!("SyncAction: copying {}", src_file.rel.display()));

                if !self.cfg.dry_run {
                    copy_file_optimized(
                        &src_file.path,
                        &dst_file,
                        ctx,
                        processed_bytes,
                        total_bytes,
                        src_file.rel.display().to_string(),
                    )?;
                }
            }

            processed_bytes += src_file.size;
            summary.bytes_processed = processed_bytes;

            if self.cfg.dry_run {
                ctx.set_progress(
                    Progress::new(processed_bytes, total_bytes)
                        .with_message(format!("Analizado: {}", src_file.rel.display())),
                );
            }
        }

        if self.cfg.delete_extra {
            for dst_file in &dst_files {
                ctx.ensure_not_cancelled()?;

                if !src_rel_paths.contains(&dst_file.rel) {
                    summary.deleted += 1;
                    ctx.info(format!(
                        "SyncAction: deleting extra {}",
                        dst_file.rel.display()
                    ));

                    if !self.cfg.dry_run {
                        fs::remove_file(&dst_file.path)?;
                    }
                }
            }
        }

        ctx.set_progress(Progress::new(total_bytes, total_bytes).with_message(
            if self.cfg.dry_run {
                "Dry-run completado"
            } else {
                "Sync completado"
            },
        ));

        ctx.info(format!(
            "SyncAction: OK | copied={} updated={} skipped={} deleted={} planned_bytes={} processed_bytes={} dry_run={}",
            summary.copied,
            summary.updated,
            summary.skipped,
            summary.deleted,
            summary.bytes_planned,
            summary.bytes_processed,
            self.cfg.dry_run
        ));

        Ok(())
    }
}

fn validate_sync_paths(src: &Path, dst: &Path) -> Result<()> {
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

    let src_abs = fs::canonicalize(src)?;
    let dst_abs = if dst.exists() {
        fs::canonicalize(dst)?
    } else {
        normalize_path(dst)?
    };

    if paths_equal(&src_abs, &dst_abs) {
        return Err(FileFlowError::Message(format!(
            "Source and destination must be different directories: {}",
            src.display()
        )));
    }

    if path_is_within(&dst_abs, &src_abs) {
        return Err(FileFlowError::Message(format!(
            "Destination cannot be inside source. This could create recursive sync loops: {} -> {}",
            src.display(),
            dst.display()
        )));
    }

    if path_is_within(&src_abs, &dst_abs) {
        return Err(FileFlowError::Message(format!(
            "Source cannot be inside destination. Choose separate directories: {} -> {}",
            src.display(),
            dst.display()
        )));
    }

    Ok(())
}

fn read_files(dir: &Path, recursive: bool) -> Result<Vec<FileEntry>> {
    let mut files = Vec::new();

    if recursive {
        for entry in WalkDir::new(dir).follow_links(false) {
            let entry = entry.map_err(|e| FileFlowError::Message(e.to_string()))?;
            let path = entry.path();

            if path.is_file() {
                files.push(build_file_entry(dir, path)?);
            }
        }
    } else {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                files.push(build_file_entry(dir, &path)?);
            }
        }
    }

    files.sort_by(|left, right| left.rel.cmp(&right.rel));
    Ok(files)
}

fn build_file_entry(base: &Path, path: &Path) -> Result<FileEntry> {
    let metadata = fs::metadata(path)?;

    Ok(FileEntry {
        path: path.to_path_buf(),
        rel: relative_key(base, path)?,
        size: metadata.len(),
        modified: modified_or_epoch(&metadata),
    })
}

fn calculate_total_bytes(files: &[FileEntry]) -> Result<u64> {
    Ok(files.iter().map(|file| file.size).sum())
}

fn build_relative_map(files: &[FileEntry]) -> HashMap<PathBuf, FileEntry> {
    let mut map = HashMap::new();

    for file in files {
        map.insert(file.rel.clone(), file.clone());
    }

    map
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

fn should_copy(src: &FileEntry, dst: &FileEntry) -> bool {
    if src.size != dst.size {
        return true;
    }

    src.modified > dst.modified
}

fn modified_or_epoch(meta: &fs::Metadata) -> SystemTime {
    meta.modified().unwrap_or(SystemTime::UNIX_EPOCH)
}
