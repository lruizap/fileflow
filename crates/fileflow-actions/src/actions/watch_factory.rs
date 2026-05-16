use std::path::PathBuf;

use fileflow_core::{Action, Result};

use crate::actions::watch::{WatchAction, WatchConfig};
use crate::args::ParsedArgs;
use crate::factory::ActionFactory;

pub struct WatchFactory;

impl ActionFactory for WatchFactory {
    fn name(&self) -> &'static str {
        "watch"
    }

    fn help(&self) -> &'static str {
        "watch -> --path <dir> --config <pipeline.json> [--recursive] [--once] [--debounce-ms <ms>]"
    }

    fn build(&self, args: &[String]) -> Result<Box<dyn Action>> {
        let parsed = ParsedArgs::from_vec(args)?;

        let path = PathBuf::from(parsed.require_str("path")?);
        let config = PathBuf::from(parsed.require_str("config")?);
        let recursive = parsed.has_flag("recursive");
        let once = parsed.has_flag("once");

        let debounce_ms = parsed
            .get_str("debounce-ms")
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(500);

        let cfg = WatchConfig {
            path,
            config,
            recursive,
            once,
            debounce_ms,
        };

        Ok(Box::new(WatchAction::new(cfg)))
    }
}
