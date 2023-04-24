use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
// 入力値が正常だったとき(srtで出力)
fn normal_cevioai() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:01,445\nこんにちは\n\n2\n00:00:01,446 --> 00:00:04,521\nこのダミーにはなんでも書いてあるの\n\n";

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd.arg("cevioai").arg("tests/data/tts/cevioai").assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}
