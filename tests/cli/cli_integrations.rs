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