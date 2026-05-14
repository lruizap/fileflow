use std::path::PathBuf;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::{Duration, Instant};

use fileflow_core::{Action, Context, FileFlowError, Result};
use notify::{recommended_watcher, Event, RecursiveMode, Watcher};

use crate::config_builder::build_pipeline_from_config_file;

#[derive(Debug, Clone)]
pub struct WatchConfig {
    pub path: PathBuf,
    pub config: PathBuf,
    pub recursive: bool,
    pub once: bool,
    pub debounce_ms: u64,
}

pub struct WatchAction {
    cfg: WatchConfig,
}

impl WatchAction {
    pub fn new(cfg: WatchConfig) -> Self {
        Self { cfg }
    }
}

impl Action for WatchAction {
    fn name(&self) -> &'static str {
        "watch"
    }

    fn execute(&self, ctx: &mut Context) -> Result<()> {
        validate_watch_config(&self.cfg)?;

        ctx.info(format!(
            "WatchAction: watching {} | recursive={} | config={}",
            self.cfg.path.display(),
            self.cfg.recursive,
            self.cfg.config.display()
        ));

        if self.cfg.once {
            ctx.info("WatchAction: --once activo, ejecutando pipeline una vez y saliendo");
            run_config_pipeline(ctx, &self.cfg.config)?;
            return Ok(());
        }

        let (tx, rx) = channel();

        let mut watcher = recommended_watcher(move |res: notify::Result<Event>| {
            let _ = tx.send(res);
        })
        .map_err(|e| FileFlowError::Message(format!("Could not create watcher: {e}")))?;

        let mode = if self.cfg.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        watcher
            .watch(&self.cfg.path, mode)
            .map_err(|e| FileFlowError::Message(format!("Could not watch path: {e}")))?;

        let mut last_run = Instant::now() - Duration::from_millis(self.cfg.debounce_ms);

        loop {
            ctx.ensure_not_cancelled()?;

            match rx.recv_timeout(Duration::from_millis(250)) {
                Ok(Ok(event)) => {
                    ctx.info(format!("WatchAction: event {:?}", event.kind));

                    if last_run.elapsed() >= Duration::from_millis(self.cfg.debounce_ms) {
                        run_config_pipeline(ctx, &self.cfg.config)?;
                        last_run = Instant::now();
                    } else {
                        ctx.info("WatchAction: event ignored by debounce");
                    }
                }
                Ok(Err(e)) => {
                    ctx.warn(format!("WatchAction: watcher error: {e}"));
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(FileFlowError::Message(
                        "Watcher channel disconnected".to_string(),
                    ));
                }
            }
        }
    }
}

fn validate_watch_config(cfg: &WatchConfig) -> Result<()> {
    if !cfg.path.exists() {
        return Err(FileFlowError::Message(format!(
            "Watch path does not exist: {}",
            cfg.path.display()
        )));
    }

    if !cfg.path.is_dir() {
        return Err(FileFlowError::Message(format!(
            "Watch path is not a directory: {}",
            cfg.path.display()
        )));
    }

    if !cfg.config.exists() {
        return Err(FileFlowError::Message(format!(
            "Pipeline config does not exist: {}",
            cfg.config.display()
        )));
    }

    if !cfg.config.is_file() {
        return Err(FileFlowError::Message(format!(
            "Pipeline config is not a file: {}",
            cfg.config.display()
        )));
    }

    Ok(())
}

fn run_config_pipeline(ctx: &mut Context, config_path: &PathBuf) -> Result<()> {
    ctx.info(format!(
        "WatchAction: running config {}",
        config_path.display()
    ));

    let pipeline = build_pipeline_from_config_file(config_path)?;
    pipeline.execute(ctx)?;

    ctx.info("WatchAction: pipeline finished");
    Ok(())
}
