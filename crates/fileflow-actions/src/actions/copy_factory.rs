use std::path::PathBuf;

use fileflow_core::{Action, Result};

use crate::args::ParsedArgs;
use crate::factory::ActionFactory;
use crate::actions::copy::{CopyAction, CopyConfig};

pub struct CopyFactory;

impl ActionFactory for CopyFactory {
    fn name(&self) -> &'static str {
        "copy"
    }

    fn help(&self) -> &'static str {
        "copy  -> --src <file> --dst <file> [--overwrite]"
    }

    fn build(&self, args: &[String]) -> Result<Box<dyn Action>> {
        let parsed = ParsedArgs::from_vec(args)?;

        let src = PathBuf::from(parsed.require_str("src")?);
        let dst = PathBuf::from(parsed.require_str("dst")?);

        // --overwrite (boolean)
        let overwrite = parsed.has_flag("overwrite");

        let cfg = CopyConfig { src, dst, overwrite };
        Ok(Box::new(CopyAction::new(cfg)))
    }
}
