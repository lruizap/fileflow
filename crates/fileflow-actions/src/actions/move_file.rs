use std::fs;
use std::path::PathBuf;

use fileflow_core::{Action, Context, Progress, Result};

use crate::fs::helpers::{
    prepare_destination,
    remove_destination_if_overwrite,
    validate_source_file,
};

#[derive(Debug, Clone)]
pub struct MoveConfig {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub overwrite: bool,
}

pub struct MoveAction {
    cfg: MoveConfig,
}

impl MoveAction {
    pub fn new(cfg: MoveConfig) -> Self {
        Self { cfg }
    }
}

impl Action for MoveAction {
    fn name(&self) -> &'static str {
        "move"
    }

    fn execute(&self, ctx: &mut Context) -> Result<()> {
        ctx.ensure_not_cancelled()?;

        let src = &self.cfg.src;
        let dst = &self.cfg.dst;

        ctx.info(format!("MoveAction: {} -> {}", src.display(), dst.display()));
        ctx.set_progress(Progress::new(0, 2).with_message("Validando..."));

        validate_source_file(src)?;
        prepare_destination(dst, self.cfg.overwrite)?;
        remove_destination_if_overwrite(dst, self.cfg.overwrite)?;

        ctx.ensure_not_cancelled()?;
        ctx.info("MoveAction: moviendo archivo...");
        ctx.set_progress(Progress::new(1, 2).with_message("Moviendo..."));

        match fs::rename(src, dst) {
            Ok(_) => {
                ctx.info("MoveAction: rename directo completado");
            }
            Err(_) => {
                ctx.warn("MoveAction: rename falló, usando fallback copy + delete");
                fs::copy(src, dst)?;
                fs::remove_file(src)?;
            }
        }

        ctx.set_progress(Progress::new(2, 2).with_message("Movido"));
        ctx.info("MoveAction: OK");

        Ok(())
    }
}