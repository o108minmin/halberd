use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
// 入力値が正常だったとき(srtで出力)
fn normal_voicevox() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:01,600\n僕はダミーなのだ\n\n2\n00:00:01,601 --> 00:00:03,179\nこれはダミー音声っす\n\n";

    let mut cmd = Command::cargo_bin("cli").unwrap();
    let assert = cmd.arg("coefont").arg("tests/data/tts/voicevox").assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}
