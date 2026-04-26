use std::fs;
use std::path::Path;

use fileflow_core::{Action, FileFlowError, Result};

use crate::actions::pipeline::PipelineAction;
use crate::config::PipelineConfig;
use crate::registry::build_action;

pub fn load_pipeline_config(path: impl AsRef<Path>) -> Result<PipelineConfig> {
    let path = path.as_ref();

    let raw = fs::read_to_string(path).map_err(|e| {
        FileFlowError::Message(format!(
            "Could not read config file '{}': {}",
            path.display(),
            e
        ))
    })?;

    let config: PipelineConfig = serde_json::from_str(&raw).map_err(|e| {
        FileFlowError::Message(format!(
            "Invalid JSON in config file '{}': {}",
            path.display(),
            e
        ))
    })?;

    validate_pipeline_config(&config)?;
    Ok(config)
}

pub fn build_pipeline_from_config(config: &PipelineConfig) -> Result<Box<dyn Action>> {
    if config.steps.is_empty() {
        return Err(FileFlowError::Message(
            "Pipeline config requires at least one step".to_string(),
        ));
    }

    let mut steps: Vec<Box<dyn Action>> = Vec::new();

    for step in &config.steps {
        if step.action == "pipeline" {
            return Err(FileFlowError::Message(
                "Nested pipeline is not allowed".to_string(),
            ));
        }

        let action = build_action(&step.action, &step.args)?;
        steps.push(action);
    }

    Ok(Box::new(PipelineAction::new(steps)))
}

pub fn build_pipeline_from_config_file(path: impl AsRef<Path>) -> Result<Box<dyn Action>> {
    let config = load_pipeline_config(path)?;
    build_pipeline_from_config(&config)
}

fn validate_pipeline_config(config: &PipelineConfig) -> Result<()> {
    if config.name.trim().is_empty() {
        return Err(FileFlowError::Message(
            "Pipeline config 'name' cannot be empty".to_string(),
        ));
    }

    if config.steps.is_empty() {
        return Err(FileFlowError::Message(
            "Pipeline config requires at least one step".to_string(),
        ));
    }

    for (idx, step) in config.steps.iter().enumerate() {
        if step.action.trim().is_empty() {
            return Err(FileFlowError::Message(format!(
                "Step {} has an empty action name",
                idx
            )));
        }
    }

    Ok(())
}
