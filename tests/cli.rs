use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn exits_0_with_msg_on_newline() {
    let mut cmd = Command::cargo_bin("treehouse").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("exiting"))
        .stdout(predicate::str::contains("\n"));
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_okay() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn hello_output() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello!\n");
}
