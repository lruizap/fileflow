use std::fs;
use std::path::PathBuf;

use fileflow_core::{Action, Context, Progress, Result};

use crate::fs::helpers::{
    copy_file_optimized, ensure_distinct_paths, prepare_destination, validate_source_file,
};

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

        ctx.info(format!(
            "CopyAction: {} -> {}",
            src.display(),
            dst.display()
        ));

        validate_source_file(src)?;
        ensure_distinct_paths(src, dst, "copy")?;
        prepare_destination(dst, self.cfg.overwrite)?;

        let total = fs::metadata(src)?.len().max(1);

        ctx.set_progress(
            Progress::new(0, total).with_message(format!("Copiando {}", src.display())),
        );

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

        ctx.set_progress(Progress::new(total, total).with_message("Copia completada"));
        ctx.info("CopyAction: OK");

        Ok(())
    }
}
