use fileflow_core::Action;

use crate::actions::{copy::{CopyAction, CopyConfig}, echo::EchoAction};

pub fn list_actions() -> Vec<&'static str> {
    vec!["echo", "copy"]
}

/// Registry por nombre (acciones sin config o genéricas)
pub fn get_action(name: &str) -> Option<Box<dyn Action>> {
    match name {
        "echo" => Some(Box::new(EchoAction)),
        _ => None,
    }
}

/// Builder específico para copy (porque necesita config)
pub fn build_copy_action(cfg: CopyConfig) -> Box<dyn Action> {
    Box::new(CopyAction::new(cfg))
}
