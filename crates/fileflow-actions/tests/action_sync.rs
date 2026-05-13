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
