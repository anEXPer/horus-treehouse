use assert_cmd::Command;

#[test]
fn works() {
    let mut cmd = Command::cargo_bin("treehouse").unwrap();
    let res = cmd.output();
    cmd.assert().success();
}
