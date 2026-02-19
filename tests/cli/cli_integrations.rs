use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn cli_actions_list_shows_echo_and_copy() {
    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args(["actions", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("echo"))
        .stdout(predicate::str::contains("copy"));
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
fn cli_run_copy_copies_file() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.txt");
    let dst = dir.path().join("out").join("dst.txt");

    fs::write(&src, "hola").unwrap();

    let src_s = src.to_string_lossy().to_string();
    let dst_s = dst.to_string_lossy().to_string();

    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args([
        "run",
        "copy",
        "--",
        "--src",
        &src_s,
        "--dst",
        &dst_s,
        "--overwrite",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("SUCCESS"));

    assert_eq!(fs::read_to_string(&dst).unwrap(), "hola");
}
