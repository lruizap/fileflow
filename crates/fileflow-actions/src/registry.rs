use fileflow_core::{Action, FileFlowError, Result};

use crate::factory::ActionFactory;
use crate::actions::copy_factory::CopyFactory;
use crate::actions::echo_factory::EchoFactory;

fn factories() -> Vec<Box<dyn ActionFactory>> {
    vec![
        Box::new(EchoFactory),
        Box::new(CopyFactory),
    ]
}

pub fn list_actions() -> Vec<&'static str> {
    factories().into_iter().map(|f| f.name()).collect()
}

pub fn list_actions_help() -> Vec<(&'static str, &'static str)> {
    factories().into_iter().map(|f| (f.name(), f.help())).collect()
}

pub fn build_action(name: &str, args: &[String]) -> Result<Box<dyn Action>> {
    for f in factories() {
        if f.name() == name {
            return f.build(args);
        }
    }
    Err(FileFlowError::Message(format!(
        "Action not found: {name}"
    )))
}
