use std::collections::HashMap;

use fileflow_core::{Action, FileFlowError, Result};

use crate::actions::pipeline::PipelineAction;
use crate::factory::ActionFactory;
use crate::registry::build_action;

pub struct PipelineFactory;

impl ActionFactory for PipelineFactory {
    fn name(&self) -> &'static str {
        "pipeline"
    }

    fn help(&self) -> &'static str {
        "pipeline  -> --step <action> [--step <action> ...] [--step-args <step:arg1,arg2>]"
    }

    fn build(&self, args: &[String]) -> Result<Box<dyn Action>> {
        let (step_names, step_args_map) = parse_pipeline_args(args)?;

        if step_names.is_empty() {
            return Err(FileFlowError::Message(
                "Pipeline requires at least one --step <action>".to_string(),
            ));
        }

        let mut steps: Vec<Box<dyn Action>> = Vec::new();

        for step_name in step_names {
            if step_name == "pipeline" {
                return Err(FileFlowError::Message(
                    "Nested pipeline is not allowed in this MVP".to_string(),
                ));
            }

            let step_args = step_args_map.get(&step_name).cloned().unwrap_or_default();
            let step = build_action(&step_name, &step_args)?;
            steps.push(step);
        }

        Ok(Box::new(PipelineAction::new(steps)))
    }
}

fn parse_pipeline_args(
    args: &[String],
) -> Result<(Vec<String>, HashMap<String, Vec<String>>)> {
    let mut step_names = Vec::new();
    let mut step_args_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--step" => {
                if i + 1 >= args.len() {
                    return Err(FileFlowError::Message(
                        "Missing value after --step".to_string(),
                    ));
                }
                step_names.push(args[i + 1].clone());
                i += 2;
            }
            "--step-args" => {
                if i + 1 >= args.len() {
                    return Err(FileFlowError::Message(
                        "Missing value after --step-args".to_string(),
                    ));
                }

                let raw = &args[i + 1];
                let (step_name, raw_args) = raw.split_once(':').ok_or_else(|| {
                    FileFlowError::Message(
                        "Invalid --step-args format. Expected <step:arg1,arg2,...>".to_string(),
                    )
                })?;

                let parsed_args = split_step_args(raw_args);
                step_args_map.insert(step_name.to_string(), parsed_args);

                i += 2;
            }
            other => {
                return Err(FileFlowError::Message(format!(
                    "Invalid pipeline argument '{}'",
                    other
                )));
            }
        }
    }

    Ok((step_names, step_args_map))
}

fn split_step_args(raw: &str) -> Vec<String> {
    let mut out = Vec::new();

    for token in raw.split(',').filter(|s| !s.trim().is_empty()) {
        let token = token.trim();

        if let Some((flag, value)) = token.split_once('=') {
            out.push(flag.to_string());
            out.push(value.to_string());
        } else {
            out.push(token.to_string());
        }
    }

    out
}