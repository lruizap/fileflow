use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_actions_list_shows_echo() {
    let mut cmd = Command::cargo_bin("fileflow-cli").expect("binary should build");
    cmd.args(["actions", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("echo"));
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
