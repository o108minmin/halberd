//! srtファイル関係のモジュール
use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::result::Result;

use chrono::prelude::*;
use chrono::Duration;
use chrono::Utc;

pub struct UnitSubRip {
    pub duration: Duration,
    pub serif: String,
}

impl fmt::Display for UnitSubRip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "duration: {}, serif: {}", self.duration, self.serif)
    }
}

#[derive(Debug)]
struct SrtError(String);

impl fmt::Display for SrtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SrtError: {}", self.0)
    }
}

impl Error for SrtError {}

/// 引数wに対して、vecをstrファイルとして出力する
pub fn output_srt<W: Write>(w: &mut W ,vec: Vec<UnitSubRip>) -> Result<(), Box<dyn Error>> {
    let mut cursor = Utc.timestamp(0, 0);
    let mut counter = 1;
    info!("Print UnitSubRips");
    debug!("input vec length: {}", vec.len());
    for i in vec.iter() {
        info!("{}", i);
        writeln!(w, "{}", counter)?;
        writeln!(
            w,
            "{} --> {}",
            cursor.format("%H:%M:%S,%3f"),
            (cursor + i.duration).format("%H:%M:%S,%3f")
        )?;
        writeln!(w, "{}", i.serif)?;
        writeln!(w)?;
        if i.duration > chrono::Duration::zero() {
            cursor = cursor + i.duration;
        } else {
            return Err(Box::new(SrtError("invalid duration: duration must be positive".into())))
        }
        counter += 1;
        cursor = cursor + Duration::milliseconds(1);
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
        input.push(UnitSubRip{
            duration: chrono::Duration::seconds(1),
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
        input.push(UnitSubRip{
            duration: chrono::Duration::seconds(-1),
            serif: String::from("negative duration"),
        });
        assert!(output_srt(&mut buf, input).is_err(), "invalid duration: duration must be positive");
    }
}
