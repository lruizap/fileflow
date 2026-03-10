use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn cli_actions_list_shows_echo_copy_move_and_pipeline() {
    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args(["actions", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("echo"))
        .stdout(predicate::str::contains("copy"))
        .stdout(predicate::str::contains("move"))
        .stdout(predicate::str::contains("pipeline"));
}

#[test]
fn cli_run_echo_success_and_logs() {
    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args(["run", "echo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SUCCESS"))
        .stdout(predicate::str::contains("EchoAction"));
}

#[test]
fn cli_run_move_moves_file() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("out").join("dst.txt");

    fs::write(&src, "hola").unwrap();

    let src_s = src.to_string_lossy().to_string();
    let dst_s = dst.to_string_lossy().to_string();

    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args([
        "run",
        "move",
        "--",
        "--src",
        &src_s,
        "--dst",
        &dst_s,
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("SUCCESS"));

    assert!(!src.exists());
    assert_eq!(fs::read_to_string(&dst).unwrap(), "hola");
}

#[test]
fn cli_run_pipeline_with_move_step_args_success() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("dst.txt");

    fs::write(&src, "hola pipeline").unwrap();

    let step_args = format!(
        "move:--src={},--dst={}",
        src.to_string_lossy(),
        dst.to_string_lossy()
    );

    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args([
        "run",
        "pipeline",
        "--",
        "--step",
        "move",
        "--step-args",
        &step_args,
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("SUCCESS"))
    .stdout(predicate::str::contains("PipelineAction"));

    assert!(!src.exists());
    assert_eq!(fs::read_to_string(&dst).unwrap(), "hola pipeline");
}