use std::fs;

use fileflow_actions::build_action;
use fileflow_core::{Engine, JobStatus};

#[test]
fn move_action_moves_file_successfully() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("out").join("dst.txt");

    fs::write(&src, "hola").unwrap();

    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        dst.to_string_lossy().to_string(),
    ];

    let act = build_action("move", &args).expect("move should build");
    let engine = Engine::new();
    let out = engine.run_action(act.as_ref());

    assert!(matches!(out.job.status, JobStatus::Success));
    assert!(!src.exists());
    assert_eq!(fs::read_to_string(&dst).unwrap(), "hola");
}

#[test]
fn move_action_rejects_moving_file_onto_itself_even_with_overwrite() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.txt");

    fs::write(&src, "keep me").unwrap();

    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        src.to_string_lossy().to_string(),
        "--overwrite".to_string(),
    ];

    let act = build_action("move", &args).expect("move should build");
    let engine = Engine::new();
    let out = engine.run_action(act.as_ref());

    assert!(matches!(out.job.status, JobStatus::Failed(_)));
    assert_eq!(fs::read_to_string(&src).unwrap(), "keep me");
}
