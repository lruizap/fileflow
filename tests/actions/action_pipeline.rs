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
    assert!(out.logs.iter().any(|l| l.message.contains("step 1/2 -> echo")));
    assert!(out.logs.iter().any(|l| l.message.contains("step 2/2 -> echo")));
    assert!(out.logs.iter().any(|l| l.message.contains("PipelineAction: finalizado correctamente")));

    let progress = out.job.progress.expect("pipeline should set final progress");
    assert_eq!(progress.current, 2);
    assert_eq!(progress.total, 2);
}

#[test]
fn pipeline_requires_at_least_one_step() {
    let args: Vec<String> = vec![];

    let err = build_action("pipeline", &args).unwrap_err();
    assert!(err.to_string().contains("at least one"));
}

#[test]
fn pipeline_rejects_nested_pipeline() {
    let args = vec![
        "--step".to_string(),
        "pipeline".to_string(),
    ];

    let err = build_action("pipeline", &args).unwrap_err();
    assert!(err.to_string().contains("Nested pipeline"));
}