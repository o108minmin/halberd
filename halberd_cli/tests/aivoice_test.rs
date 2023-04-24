use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
// 入力値が正常だったとき(srtで出力)
fn normal_aivoice() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:01,838\nこれからよろしく\n\n2\n00:00:01,839 --> 00:00:03,328\nこんにちは\n\n";

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd
        .arg("aivoice")
        .arg("tests/data/tts/aivoice")
        .assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}
