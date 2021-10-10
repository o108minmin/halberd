use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn normal_voiceroid() -> Result<(), Box<dyn std::error::Error>>  {
    let expected = "1\n00:00:00,000 --> 00:00:01,269\nテスト\n\n2\n00:00:01,270 --> 00:00:06,295\nこんにちは、だみーです。17歳です\n\n";

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("voiceroid").arg("tests/data/softwaretalk/vocieroid").assert();
    assert
        .success()
        .stdout(predicate::str::contains(expected));
    Ok(())
}
