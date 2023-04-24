use std::boxed::Box;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
// 入力値が正常だったとき(指定したディレクトリが空)
fn normal_empty_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd.arg("coefont").arg(dir.path()).assert();
    assert.success();
    Ok(())
}

#[test]
// 入力値が正常だったとき(wavファイルは存在するが、txtファイルが存在しない)
fn normal_txt_file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    //let assert = cmd.arg("coefont").arg("coefont").arg("tests/data/normal/coefont").assert();
    let assert = cmd
        .arg("coefont")
        .arg("tests/data/normal/coefont_no_txt")
        .assert();
    assert.success();
    Ok(())
}

#[test]
// 入力値が正常だったとき(format指定)
fn normal_enable_format() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd.arg("coefont").arg(dir.path()).arg("-fsrt").assert();
    assert.success();
    Ok(())
}

#[test]
// 入力値が異常だったとき(TTSが指定されていない)
fn error_required_tts_talk_type() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd.assert();
    assert.failure().stderr(predicate::str::contains("TTS"));
    Ok(())
}

#[test]
// 入力値が異常だったとき(不正なTTSが指定されている)
fn error_invalid_tts_talk_type() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd.arg("sample").assert();
    assert.failure().stderr(predicate::str::contains("TTS"));
    Ok(())
}

#[test]
// 入力値が異常だったとき(入力元に指定したディレクトリが存在しない)
fn error_input_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd.arg("coefont").arg("./notfound").assert();
    assert
        .failure()
        .stderr(predicate::str::contains("Problem opening input directory"));
    Ok(())
}

#[test]
// 入力値が異常だったとき(出力先に指定したディレクトリが存在しない)
fn error_output_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd
        .arg("coefont")
        .arg("tests/data/normal/coefont")
        .arg("-onotfound/output.srt")
        .assert();
    assert
        .failure()
        .stderr(predicate::str::contains("Problem can\'t open file:"));
    Ok(())
}
