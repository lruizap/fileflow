use std::fs;
use std::path::{Path, PathBuf};

use fileflow_core::{Action, Context, FileFlowError, Progress, Result};

#[derive(Debug, Clone)]
pub struct CopyConfig {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub overwrite: bool,
}

pub struct CopyAction {
    cfg: CopyConfig,
}

impl CopyAction {
    pub fn new(cfg: CopyConfig) -> Self {
        Self { cfg }
    }
}

impl Action for CopyAction {
    fn name(&self) -> &'static str {
        "copy"
    }

    fn execute(&self, ctx: &mut Context) -> Result<()> {
        ctx.ensure_not_cancelled()?;

        let src = &self.cfg.src;
        let dst = &self.cfg.dst;

        ctx.info(format!("CopyAction: {} -> {}", src.display(), dst.display()));
        ctx.set_progress(Progress::new(0, 1).with_message("Validando..."));

        validate_source_file(src)?;
        prepare_destination(dst, self.cfg.overwrite)?;

        ctx.ensure_not_cancelled()?;
        ctx.info("CopyAction: copiando archivo...");
        fs::copy(src, dst)?;

        ctx.set_progress(Progress::new(1, 1).with_message("Copiado"));
        ctx.info("CopyAction: OK");

        Ok(())
    }
}

fn validate_source_file(src: &Path) -> Result<()> {
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

fn prepare_destination(dst: &Path, overwrite: bool) -> Result<()> {
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
