use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn normal_empty_directory() -> Result<(), Box<dyn std::error::Error>>  {
    let dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("coefontstudio").arg(dir.path()).assert();
    assert
        .success();
    Ok(())
}

#[test]
fn error_required_software_talk_type() -> Result<(), Box<dyn std::error::Error>>  {
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.assert();
    assert
        .failure()
        .stderr(predicate::str::contains("The following required arguments"))
        .stderr(predicate::str::contains("SoftwareTalkType"));
    Ok(())
}

#[test]
fn error_invalid_software_talk_type() -> Result<(), Box<dyn std::error::Error>>  {
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("sample").assert();
    assert
        .failure()
        .stderr(predicate::str::contains("isn't a valid value for"))
        .stderr(predicate::str::contains("SoftwareTalkType"));
    Ok(())
}

#[test]
fn error_txt_file_not_found() -> Result<(), Box<dyn std::error::Error>>  {
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("coefontstudio").arg("tests/data/error/coefontstudio").assert();
    assert
        .failure()
        .stderr(predicate::str::contains("TextError"));
    Ok(())
}

#[test]
fn error_input_not_found() -> Result<(), Box<dyn std::error::Error>>  {
    let mut cmd = Command::cargo_bin("halberd").unwrap();
    let assert = cmd.arg("coefontstudio").arg("./notfound").assert();
    assert
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}
