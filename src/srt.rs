//! srtファイル関係のモジュール
use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::result::Result;

use time::macros::format_description;
use time::Duration;

use crate::unitsubrip::UnitSubRip;

impl Error for SrtError {}

#[derive(Debug)]
struct SrtError(String);

impl fmt::Display for SrtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SrtError: {}", self.0)
    }
}

/// 引数wに対して、vecをsrtファイルとして出力する
pub fn output_srt<W: Write>(w: &mut W, vec: Vec<UnitSubRip>) -> Result<(), Box<dyn Error>> {
    let mut cursor = time::Time::MIDNIGHT;
    let mut counter = 1;
    let formatting = format_description!("[hour]:[minute]:[second],[subsecond digits:3]");
    info!("Print UnitSubRips");
    debug!("input vec length: {}", vec.len());
    for i in vec.iter() {
        let start_cursol = cursor.format(formatting)?;
        let end_cursol = (cursor + i.duration).format(formatting)?;
        info!("{}", i);
        writeln!(w, "{}", counter)?;
        writeln!(w, "{} --> {}", start_cursol, end_cursol)?;
        writeln!(w, "{}", i.serif)?;
        writeln!(w)?;
        if i.duration > time::Duration::ZERO {
            cursor += i.duration;
        } else {
            return Err(Box::new(SrtError(
                "invalid duration: duration must be positive".into(),
            )));
        }
        counter += 1;
        cursor += Duration::milliseconds(1);
    }
    w.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // 入力値が正常だったとき
    fn normal_output_srt() {
        let mut buf = Vec::<u8>::new();
        let mut input = Vec::<UnitSubRip>::new();
        input.push(UnitSubRip {
            duration: time::Duration::seconds(1),
            serif: String::from("say"),
        });
        let expected = b"1\n00:00:00,000 --> 00:00:01,000\nsay\n\n";
        let result = output_srt(&mut buf, input);
        assert!(result.is_ok());
        assert_eq!(buf, expected);
    }

    #[test]
    // 不正な値(負の値)が入る
    fn error_invalid_duration() {
        let mut buf = Vec::<u8>::new();
        let mut input = Vec::<UnitSubRip>::new();
        input.push(UnitSubRip {
            duration: time::Duration::seconds(-1),
            serif: String::from("negative duration"),
        });
        assert!(
            output_srt(&mut buf, input).is_err(),
            "invalid duration: duration must be positive"
        );
    }
}
