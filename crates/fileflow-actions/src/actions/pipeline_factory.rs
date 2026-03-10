use fileflow_core::{Action, FileFlowError, Result};

use crate::factory::ActionFactory;
use crate::registry::build_action;
use crate::actions::pipeline::PipelineAction;

pub struct PipelineFactory;

impl ActionFactory for PipelineFactory {
    fn name(&self) -> &'static str {
        "pipeline"
    }

    fn help(&self) -> &'static str {
        "pipeline  -> --step <action> [--step <action> ...] (MVP: steps sin args)"
    }

    fn build(&self, args: &[String]) -> Result<Box<dyn Action>> {
        let mut step_names = Vec::new();
        let mut i = 0;

        while i < args.len() {
            let token = &args[i];

            if token == "--step" {
                if i + 1 >= args.len() {
                    return Err(FileFlowError::Message(
                        "Missing value after --step".to_string(),
                    ));
                }

                let step_name = args[i + 1].clone();

                if step_name == "pipeline" {
                    return Err(FileFlowError::Message(
                        "Nested pipeline is not allowed in this MVP".to_string(),
                    ));
                }

                step_names.push(step_name);
                i += 2;
            } else {
                return Err(FileFlowError::Message(format!(
                    "Invalid pipeline argument '{}'. Expected '--step <action>'.",
                    token
                )));
            }
        }

        if step_names.is_empty() {
            return Err(FileFlowError::Message(
                "Pipeline requires at least one --step <action>".to_string(),
            ));
        }

        let mut steps: Vec<Box<dyn Action>> = Vec::new();

        for step_name in step_names {
            let step = build_action(&step_name, &[])?;
            steps.push(step);
        }

        Ok(Box::new(PipelineAction::new(steps)))
    }
}