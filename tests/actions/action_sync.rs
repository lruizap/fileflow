use std::fs;

use fileflow_actions::build_action;
use fileflow_core::{Engine, JobStatus};

#[test]
fn sync_action_copies_missing_files() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");

    fs::create_dir_all(&src).unwrap();
    fs::create_dir_all(&dst).unwrap();

    fs::write(src.join("a.txt"), "hola").unwrap();
    fs::write(src.join("b.txt"), "adios").unwrap();

    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        dst.to_string_lossy().to_string(),
    ];

    let act = build_action("sync", &args).expect("sync should build");
    let engine = Engine::new();
    let out = engine.run_action(act.as_ref());

    assert!(matches!(out.job.status, JobStatus::Success));
    assert_eq!(fs::read_to_string(dst.join("a.txt")).unwrap(), "hola");
    assert_eq!(fs::read_to_string(dst.join("b.txt")).unwrap(), "adios");
}

#[test]
fn sync_action_deletes_extra_files_when_requested() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");

    fs::create_dir_all(&src).unwrap();
    fs::create_dir_all(&dst).unwrap();

    fs::write(src.join("keep.txt"), "ok").unwrap();
    fs::write(dst.join("keep.txt"), "old").unwrap();
    fs::write(dst.join("extra.txt"), "delete me").unwrap();

    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        dst.to_string_lossy().to_string(),
        "--delete-extra".to_string(),
    ];

    let act = build_action("sync", &args).expect("sync should build");
    let engine = Engine::new();
    let out = engine.run_action(act.as_ref());

    assert!(matches!(out.job.status, JobStatus::Success));
    assert!(dst.join("keep.txt").exists());
    assert!(!dst.join("extra.txt").exists());
}