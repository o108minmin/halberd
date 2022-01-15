use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
// 入力値が正常だったとき(指定したディレクトリが空)
fn normal_empty_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("coefont").arg(dir.path()).assert();
    assert.success();
    Ok(())
}

#[test]
// 入力値が異常だったとき(TTSが指定されていない)
fn error_required_tts_talk_type() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.assert();
    assert
        .failure()
        .stderr(predicate::str::contains("The following required arguments"))
        .stderr(predicate::str::contains("TTS"));
    Ok(())
}

#[test]
// 入力値が異常だったとき(不正なTTSが指定されている)
fn error_invalid_tts_talk_type() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("sample").assert();
    assert
        .failure()
        .stderr(predicate::str::contains("isn't a valid value for"))
        .stderr(predicate::str::contains("TTS"));
    Ok(())
}

#[test]
// 入力値が異常だったとき(wavファイルは存在するが、txtファイルが存在しない)
fn error_txt_file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("coefont").arg("tests/data/error/coefont").assert();
    assert
        .failure()
        .stderr(predicate::str::contains("TextError"));
    Ok(())
}

#[test]
// 入力値が異常だったとき(指定したディレクトリが存在しない)
fn error_input_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("coefont").arg("./notfound").assert();
    assert
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}
