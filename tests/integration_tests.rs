use assert_cmd::prelude::*;
use predicates;
use std::process::Command;

#[test]
fn run_on_shelly() {
    Command::cargo_bin("mir")
        .expect("binary exists")
        .arg("tests/shelly.txt")
        .assert()
        .success()
        .stdout(predicates::str::contains("!yllehS m'I ,iH"))
        .stdout(predicates::str::contains(r"/     \  |  o |"));
}
