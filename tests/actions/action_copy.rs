use std::fs;

use fileflow_actions::build_action;
use fileflow_core::{Engine, JobStatus};

#[test]
fn copy_action_copies_file_and_overwrite_works() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("nested").join("dst.txt");

    fs::write(&src, "hola").unwrap();

    // 1) Copia normal
    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        dst.to_string_lossy().to_string(),
    ];

    let act = build_action("copy", &args).expect("copy should build");
    let engine = Engine::new();
    let out = engine.run_action(act.as_ref());

    assert!(matches!(out.job.status, JobStatus::Success));
    assert_eq!(fs::read_to_string(&dst).unwrap(), "hola");

    // 2) Sin overwrite debe fallar si existe
    let act2 = build_action("copy", &args).expect("copy should build");
    let out2 = engine.run_action(act2.as_ref());
    assert!(matches!(out2.job.status, JobStatus::Failed(_)));

    // 3) Con overwrite funciona
    fs::write(&src, "adios").unwrap();
    let args_over = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        dst.to_string_lossy().to_string(),
        "--overwrite".to_string(),
    ];

    let act3 = build_action("copy", &args_over).expect("copy should build");
    let out3 = engine.run_action(act3.as_ref());
    assert!(matches!(out3.job.status, JobStatus::Success));
    assert_eq!(fs::read_to_string(&dst).unwrap(), "adios");
}
