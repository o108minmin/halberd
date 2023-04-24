use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
// 入力値が正常だったとき(srtで出力)
fn normal_voicepeak() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:02,389\nやあ、ダミーマンだよ\n\n2\n00:00:02,390 --> 00:00:04,158\nこれからよろしく\n\n";

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd
        .arg("voicepeak")
        .arg("tests/data/tts/voicepeak")
        .assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}
