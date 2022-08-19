use std::boxed::Box;
use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
// 入力値が正常だったとき(srtで出力)
fn normal_coefont() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:03,499\n私はだみーです。よろしくお願いします。\n\n2\n00:00:03,500 --> 00:00:04,475\nこんにちわ\n\n";

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("coefont").arg("tests/data/tts/coefont").assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}

#[test]
// 入力値が正常だったとき(srtでファイルに出力)
fn normal_coefont_srt_file() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:03,499\n私はだみーです。よろしくお願いします。\n\n2\n00:00:03,500 --> 00:00:04,475\nこんにちわ\n\n";

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd
        .arg("coefont")
        .arg("tests/data/tts/coefont")
        .arg("-ooutput.srt")
        .assert();
    let result = fs::read_to_string("output.srt").unwrap();
    assert.success();
    assert_eq!(result, expected);
    Ok(())
}

#[test]
// 入力値が正常だったとき(xmlで出力)
fn normal_coefont_xml_stdout() -> Result<(), Box<dyn std::error::Error>> {
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

#[test]
// 入力値が正常だったとき(xmlでファイルに出力)
fn normal_coefont_xml_file() -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string("tests/data/xml/normal_coefont_xml.xml").unwrap();

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd
        .arg("coefont")
        .arg("tests/data/tts/coefont")
        .arg("-fxml")
        .arg("-ooutput.xml")
        .assert();
    let result = fs::read_to_string("output.xml").unwrap();
    assert.success();
    assert_eq!(result, expected);
    Ok(())
}

#[test]
// 入力値が正常だったとき(ファイル名にディレクトリ名を使用)
fn normal_coefont_file_name_using_dirname() -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string("tests/data/xml/normal_coefont_xml.xml").unwrap();

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd
        .arg("coefont")
        .arg("tests/data/tts/coefont")
        .arg("-fxml")
        .arg("-odirname")
        .assert();
    let result = fs::read_to_string("coefont.xml").unwrap();
    assert.success();
    assert_eq!(result, expected);
    Ok(())
}

#[test]
// 入力値が正常だったとき(txtが存在するがwavが存在しないlazyなフォルダを入力し、xmlでファイルに出力)
fn normal_coefont_xml_file_lazy() -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string("tests/data/xml/normal_coefont_lazy_xml.xml").unwrap();

    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd
        .arg("coefont")
        .arg("tests/data/tts/coefont_lazy")
        .arg("-fxml")
        .arg("-odirname")
        .assert();
    let result = fs::read_to_string("coefont_lazy.xml").unwrap();
    assert.success();
    assert_eq!(result, expected);
    Ok(())
}
