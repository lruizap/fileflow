use std::fs;
use std::path::PathBuf;

use fileflow_core::{Action, Context, Progress, Result};

use crate::fs::helpers::{prepare_destination, validate_source_file};

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