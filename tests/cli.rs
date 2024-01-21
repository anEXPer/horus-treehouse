use assert_cmd::Command;

#[test]
fn works() {
    let mut cmd = Command::cargo_bin("treehouse").unwrap();
    cmd.assert().success();
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
