use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn normal_coefontstudio() -> Result<(), Box<dyn std::error::Error>>  {
    let expected = "1\n00:00:00,000 --> 00:00:00,975\nこんにちわ\n\n2\n00:00:00,976 --> 00:00:04,475\n私はだみーです。よろしくお願いします。\n\n";

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("coefontstudio").arg("tests/data/softwaretalk/coefontstudio").assert();
    assert
        .success()
        .stdout(predicate::str::contains(expected));
    Ok(())
}
