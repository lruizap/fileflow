#[test]
fn cli_validate_config_json_success() {
    let dir = tempfile::tempdir().unwrap();

    let json_path = dir.path().join("pipeline.json");

    let json = r#"
{
  "name": "validate_demo",
  "steps": [
    {
      "action": "echo",
      "args": []
    }
  ]
}
"#;

    fs::write(&json_path, json).unwrap();

    let json_s = json_path.to_string_lossy().to_string();

    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args(["validate-config", &json_s])
        .assert()
        .success()
        .stdout(predicate::str::contains("Config OK"))
        .stdout(predicate::str::contains("validate_demo"))
        .stdout(predicate::str::contains("Steps: 1"));
}

#[test]
fn cli_actions_list_shows_echo_copy_move_sync_watch_and_pipeline() {
    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args(["actions", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("echo"))
        .stdout(predicate::str::contains("copy"))
        .stdout(predicate::str::contains("move"))
        .stdout(predicate::str::contains("sync"))
        .stdout(predicate::str::contains("watch"))
        .stdout(predicate::str::contains("pipeline"));
}

#[test]
fn cli_run_sync_recursive_copies_nested_files() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");

    fs::create_dir_all(src.join("nested")).unwrap();
    fs::write(src.join("nested").join("a.txt"), "nested cli").unwrap();

    let src_s = src.to_string_lossy().to_string();
    let dst_s = dst.to_string_lossy().to_string();

    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args([
        "run",
        "sync",
        "--",
        "--src",
        &src_s,
        "--dst",
        &dst_s,
        "--recursive",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("SUCCESS"));

    assert_eq!(
        fs::read_to_string(dst.join("nested").join("a.txt")).unwrap(),
        "nested cli"
    );
}