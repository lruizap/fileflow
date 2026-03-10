use std::path::PathBuf;

use fileflow_core::{Action, Result};

use crate::actions::move_file::{MoveAction, MoveConfig};
use crate::args::ParsedArgs;
use crate::factory::ActionFactory;

pub struct MoveFactory;

impl ActionFactory for MoveFactory {
    fn name(&self) -> &'static str {
        "move"
    }

    fn help(&self) -> &'static str {
        "move  -> --src <file> --dst <file> [--overwrite]"
    }

    fn build(&self, args: &[String]) -> Result<Box<dyn Action>> {
        let parsed = ParsedArgs::from_vec(args)?;

        let src = PathBuf::from(parsed.require_str("src")?);
        let dst = PathBuf::from(parsed.require_str("dst")?);
        let overwrite = parsed.has_flag("overwrite");

        let cfg = MoveConfig { src, dst, overwrite };
        Ok(Box::new(MoveAction::new(cfg)))
    }
}