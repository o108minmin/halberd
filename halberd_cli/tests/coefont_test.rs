use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn fixture_path(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

#[test]
// 入力値が正常だったとき(srtで出力)
fn normal_coefont() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:03,499\n私はだみーです。よろしくお願いします。\n\n2\n00:00:03,500 --> 00:00:04,475\nこんにちわ\n\n";

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd
        .arg("coefont")
        .arg(fixture_path("tests/data/tts/coefont"))
        .assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}

#[test]
// 入力値が正常だったとき(srtでファイルに出力)
fn normal_coefont_srt_file() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:03,499\n私はだみーです。よろしくお願いします。\n\n2\n00:00:03,500 --> 00:00:04,475\nこんにちわ\n\n";
    let output_dir = tempdir()?;
    let output = output_dir.path().join("output.srt");

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    cmd.arg("coefont")
        .arg(fixture_path("tests/data/tts/coefont"))
        .arg("-o")
        .arg(&output)
        .assert()
        .success();
    let result = fs::read_to_string(output)?;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
// 入力値が正常だったとき(xmlで出力)
fn normal_coefont_xml_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(fixture_path("tests/data/xml/normal_coefont_xml.xml"))?;

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd
        .arg("coefont")
        .arg(fixture_path("tests/data/tts/coefont"))
        .arg("-fxml")
        .assert();
    assert.success().stdout(predicate::str::contains(expected));
    Ok(())
}

#[test]
// 入力値が正常だったとき(xmlでファイルに出力)
fn normal_coefont_xml_file() -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(fixture_path("tests/data/xml/normal_coefont_xml.xml"))?;
    let output_dir = tempdir()?;
    let output = output_dir.path().join("output.xml");

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    cmd.arg("coefont")
        .arg(fixture_path("tests/data/tts/coefont"))
        .arg("-fxml")
        .arg("-o")
        .arg(&output)
        .assert()
        .success();
    let result = fs::read_to_string(output)?;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
// 入力値が正常だったとき(fcpxmlでファイルに出力)
fn normal_coefont_fcpxml_file() -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(fixture_path("tests/data/xml/normal_coefont_xml.xml"))?;
    let output_dir = tempdir()?;
    let output = output_dir.path().join("output.xml");

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    cmd.arg("coefont")
        .arg(fixture_path("tests/data/tts/coefont"))
        .arg("-ffcpxml")
        .arg("-o")
        .arg(&output)
        .assert()
        .success();
    let result = fs::read_to_string(output)?;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
// 入力値が正常だったとき(ファイル名にディレクトリ名を使用)
fn normal_coefont_file_name_using_dirname() -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(fixture_path("tests/data/xml/normal_coefont_xml.xml"))?;
    let output_dir = tempdir()?;

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    cmd.current_dir(output_dir.path())
        .arg("coefont")
        .arg(fixture_path("tests/data/tts/coefont"))
        .arg("-fxml")
        .arg("-odirname")
        .assert()
        .success();
    let result = fs::read_to_string(output_dir.path().join("coefont.xml"))?;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
// 入力値が正常だったとき(txtが存在するがwavが存在しないlazyなフォルダを入力し、xmlでファイルに出力)
fn normal_coefont_xml_file_lazy() -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(fixture_path("tests/data/xml/normal_coefont_lazy_xml.xml"))?;
    let output_dir = tempdir()?;

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    cmd.current_dir(output_dir.path())
        .arg("coefont")
        .arg(fixture_path("tests/data/tts/coefont_lazy"))
        .arg("-fxml")
        .arg("-odirname")
        .assert()
        .success();
    let result = fs::read_to_string(output_dir.path().join("coefont_lazy.xml"))?;
    assert_eq!(result, expected);
    Ok(())
}
