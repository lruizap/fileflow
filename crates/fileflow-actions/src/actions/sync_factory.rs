use std::path::PathBuf;

use fileflow_core::{Action, Result};

use crate::actions::sync::{SyncAction, SyncConfig};
use crate::args::ParsedArgs;
use crate::factory::ActionFactory;

pub struct SyncFactory;

impl ActionFactory for SyncFactory {
    fn name(&self) -> &'static str {
        "sync"
    }

    fn help(&self) -> &'static str {
        "sync  -> --src <dir> --dst <dir> [--delete-extra]"
    }

    fn build(&self, args: &[String]) -> Result<Box<dyn Action>> {
        let parsed = ParsedArgs::from_vec(args)?;

        let src = PathBuf::from(parsed.require_str("src")?);
        let dst = PathBuf::from(parsed.require_str("dst")?);
        let delete_extra = parsed.has_flag("delete-extra");
        let overwrite = parsed.has_flag("overwrite");

        let cfg = SyncConfig {
            src,
            dst,
            delete_extra,
            overwrite,
        };

        Ok(Box::new(SyncAction::new(cfg)))
    }
}
