use std::boxed::Box;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
// 入力値が正常だったとき
fn normal_softalk() -> Result<(), Box<dyn std::error::Error>> {
    let expected = "1\n00:00:00,000 --> 00:00:01,837\nゆっくりしていってね\n\n2\n00:00:01,838 --> 00:00:03,080\nダミーだぜ\n\n";

    let mut cmd = Command::cargo_bin("halberdcli").unwrap();
    let assert = cmd.arg("softalk").arg("tests/data/tts/softalk").assert();
    match assert
        .success()
        .try_stdout(predicate::str::contains(expected))
    {
        Ok(_) => Ok(()),
        Err(_) => {
            // 丸めの影響が考えられるので再実行 080 -> 081
            let expected = "1\n00:00:00,000 --> 00:00:01,837\nゆっくりしていってね\n\n2\n00:00:01,838 --> 00:00:03,081\nダミーだぜ\n\n";

            let mut cmd = Command::cargo_bin("halberdcli").unwrap();
            let assert = cmd.arg("softalk").arg("tests/data/tts/softalk").assert();
            assert.success().stdout(predicate::str::contains(expected));
            Ok(())
        }
    }
}
