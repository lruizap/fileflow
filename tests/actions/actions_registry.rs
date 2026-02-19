use fileflow_actions::{get_action, list_actions};
use fileflow_core::{Engine, JobStatus};

#[test]
fn list_actions_contains_echo() {
    let actions = list_actions();
    assert!(actions.contains(&"echo"));
}

#[test]
fn get_action_returns_echo_and_engine_runs_it() {
    let action = get_action("echo").expect("echo action should exist");
    let engine = Engine::new();

    let out = engine.run_action(action.as_ref());

    assert_eq!(out.job.action_name, "echo");
    assert!(matches!(out.job.status, JobStatus::Success));
    assert!(!out.logs.is_empty());
}
