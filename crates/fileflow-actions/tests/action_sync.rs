#[test]
fn sync_action_recursive_copies_nested_files() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");

    std::fs::create_dir_all(src.join("nested/deep")).unwrap();

    std::fs::write(src.join("root.txt"), "root").unwrap();
    std::fs::write(src.join("nested").join("a.txt"), "nested").unwrap();
    std::fs::write(src.join("nested/deep").join("b.txt"), "deep").unwrap();

    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        dst.to_string_lossy().to_string(),
        "--recursive".to_string(),
    ];

    let act = fileflow_actions::build_action("sync", &args).expect("sync should build");
    let engine = fileflow_core::Engine::new();
    let out = engine.run_action(act.as_ref());

    assert!(matches!(out.job.status, fileflow_core::JobStatus::Success));
    assert_eq!(
        std::fs::read_to_string(dst.join("root.txt")).unwrap(),
        "root"
    );
    assert_eq!(
        std::fs::read_to_string(dst.join("nested").join("a.txt")).unwrap(),
        "nested"
    );
    assert_eq!(
        std::fs::read_to_string(dst.join("nested/deep").join("b.txt")).unwrap(),
        "deep"
    );
}

#[test]
fn sync_action_dry_run_reports_changes_without_writing() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");

    std::fs::create_dir_all(&src).unwrap();
    std::fs::create_dir_all(&dst).unwrap();
    std::fs::write(src.join("new.txt"), "new").unwrap();
    std::fs::write(dst.join("extra.txt"), "extra").unwrap();

    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        dst.to_string_lossy().to_string(),
        "--delete-extra".to_string(),
        "--dry-run".to_string(),
    ];

    let act = fileflow_actions::build_action("sync", &args).expect("sync should build");
    let engine = fileflow_core::Engine::new();
    let out = engine.run_action(act.as_ref());

    assert!(matches!(out.job.status, fileflow_core::JobStatus::Success));
    assert!(!dst.join("new.txt").exists());
    assert!(dst.join("extra.txt").exists());
    assert!(out.logs.iter().any(|l| l.message.contains("dry_run=true")));
}

#[test]
fn sync_action_dry_run_does_not_create_missing_destination() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("missing-dst");

    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("new.txt"), "new").unwrap();

    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        dst.to_string_lossy().to_string(),
        "--dry-run".to_string(),
    ];

    let act = fileflow_actions::build_action("sync", &args).expect("sync should build");
    let engine = fileflow_core::Engine::new();
    let out = engine.run_action(act.as_ref());

    assert!(matches!(out.job.status, fileflow_core::JobStatus::Success));
    assert!(!dst.exists());
}

#[test]
fn sync_action_rejects_destination_inside_source() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = src.join("backup");

    std::fs::create_dir_all(&src).unwrap();

    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        dst.to_string_lossy().to_string(),
        "--recursive".to_string(),
    ];

    let act = fileflow_actions::build_action("sync", &args).expect("sync should build");
    let engine = fileflow_core::Engine::new();
    let out = engine.run_action(act.as_ref());

    match out.job.status {
        fileflow_core::JobStatus::Failed(message) => {
            assert!(message.contains("Destination cannot be inside source"));
        }
        other => panic!("expected Failed, got {other:?}"),
    }
}

#[test]
fn sync_action_rejects_same_source_and_destination() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");

    std::fs::create_dir_all(&src).unwrap();

    let args = vec![
        "--src".to_string(),
        src.to_string_lossy().to_string(),
        "--dst".to_string(),
        src.to_string_lossy().to_string(),
    ];

    let act = fileflow_actions::build_action("sync", &args).expect("sync should build");
    let engine = fileflow_core::Engine::new();
    let out = engine.run_action(act.as_ref());

    match out.job.status {
        fileflow_core::JobStatus::Failed(message) => {
            assert!(message.contains("Source and destination must be different"));
        }
        other => panic!("expected Failed, got {other:?}"),
    }
}
