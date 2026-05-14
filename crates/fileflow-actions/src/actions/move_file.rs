use std::fs;
use std::path::PathBuf;

use fileflow_core::{Action, Context, Progress, Result};

use crate::fs::helpers::{
    copy_file_optimized, ensure_distinct_paths, prepare_destination,
    remove_destination_if_overwrite, validate_source_file,
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

        ctx.info(format!(
            "MoveAction: {} -> {}",
            src.display(),
            dst.display()
        ));

        validate_source_file(src)?;
        ensure_distinct_paths(src, dst, "move")?;
        prepare_destination(dst, self.cfg.overwrite)?;
        remove_destination_if_overwrite(dst, self.cfg.overwrite)?;

        let total = fs::metadata(src)?.len().max(1);

        ctx.set_progress(
            Progress::new(0, total).with_message(format!("Moviendo {}", src.display())),
        );

        match fs::rename(src, dst) {
            Ok(_) => {
                ctx.info("MoveAction: rename directo completado");
                ctx.set_progress(Progress::new(total, total).with_message("Movimiento completado"));
            }
            Err(_) => {
                ctx.warn("MoveAction: rename falló, usando fallback copy + delete");

                copy_file_optimized(
                    src,
                    dst,
                    ctx,
                    0,
                    total,
                    src.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("archivo"),
                )?;

                fs::remove_file(src)?;
                ctx.set_progress(Progress::new(total, total).with_message("Movimiento completado"));
            }
        }

        ctx.info("MoveAction: OK");

        Ok(())
    }
}
