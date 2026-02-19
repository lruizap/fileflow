use fileflow_actions::{build_action, list_actions};
use fileflow_core::{Engine, JobStatus};

#[test]
fn list_actions_contains_echo_and_copy() {
    let actions = list_actions();
    assert!(actions.contains(&"echo"));
    assert!(actions.contains(&"copy"));
}

#[test]
fn build_action_echo_and_run_it() {
    let act = build_action("echo", &[]).expect("echo should build");
    let engine = Engine::new();

    let out = engine.run_action(act.as_ref());
    assert_eq!(out.job.action_name, "echo");
    assert!(matches!(out.job.status, JobStatus::Success));
}
