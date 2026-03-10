use std::fs;

use fileflow_actions::build_action;
use fileflow_core::{Engine, JobStatus};

#[test]
fn pipeline_with_two_echo_steps_runs_successfully() {
    let args = vec![
        "--step".to_string(),
        "echo".to_string(),
        "--step".to_string(),
        "echo".to_string(),
    ];

    let action = build_action("pipeline", &args).expect("pipeline should build");
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    assert!(matches!(out.job.status, JobStatus::Success));
    assert!(out.logs.iter().any(|l| l.message.contains("PipelineAction: iniciando")));
}

#[test]
fn pipeline_can_run_move_with_step_args() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("dst.txt");

    fs::write(&src, "hola pipeline").unwrap();

    let step_args = format!(
        "move:--src={},--dst={}",
        src.to_string_lossy(),
        dst.to_string_lossy()
    );

    let args = vec![
        "--step".to_string(),
        "move".to_string(),
        "--step-args".to_string(),
        step_args,
    ];

    let action = build_action("pipeline", &args).expect("pipeline should build");
    let engine = Engine::new();
    let out = engine.run_action(action.as_ref());

    assert!(matches!(out.job.status, JobStatus::Success));
    assert!(!src.exists());
    assert_eq!(fs::read_to_string(&dst).unwrap(), "hola pipeline");
}

#[test]
fn pipeline_requires_at_least_one_step() {
    let args: Vec<String> = vec![];

    let err = build_action("pipeline", &args).unwrap_err();
    assert!(err.to_string().contains("at least one"));
}