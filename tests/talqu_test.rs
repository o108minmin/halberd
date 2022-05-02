use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
// 入力値が正常だったとき
fn normal_talqu() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:02,571\nこんばんは\n\n2\n00:00:02,572 --> 00:00:04,157\n君は実にDummyだな\n\n";

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("talqu").arg("tests/data/tts/talqu").assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}
