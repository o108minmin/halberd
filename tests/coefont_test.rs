use std::boxed::Box;
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
// 入力値が正常だったとき
fn normal_coefont() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:03,499\n私はだみーです。よろしくお願いします。\n\n2\n00:00:03,500 --> 00:00:04,475\nこんにちわ\n\n";

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("coefont").arg("tests/data/tts/coefont").assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}

#[test]
// 入力値が正常だったとき(xmlで出力)
fn normal_coefont_xml() -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string("tests/data/xml/normal_coefont_xml.xml").unwrap();

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd
        .arg("coefont")
        .arg("tests/data/tts/coefont")
        .arg("-fxml")
        .assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}
