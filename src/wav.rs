//! wavファイル関係のモジュール
use std::boxed::Box;
use std::fmt;
use std::{error::Error, path::PathBuf};

#[derive(Debug)]
struct WavError(String);

impl fmt::Display for WavError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WavError: {}", self.0)
    }
}

impl Error for WavError {}

/// wavファイルの秒数を計算する
/// * `reader` - 秒数を計算したいhoundのwav reader.
pub fn calculate_wave_seconds(path: PathBuf) -> Result<f64, Box<dyn Error>> {
    let reader = match hound::WavReader::open(path) {
        Ok(f) => f,
        Err(_) => return Err(Box::new(WavError("can't open wav file".into()))),
    };
    //let reader = hound::WavReader::open(path).unwrap_or(10);
    debug!("input duration: {}", reader.duration());
    debug!("input sampling rate: {}", reader.spec().sample_rate);
    debug!(
        "return {} / {}",
        reader.duration(),
        reader.spec().sample_rate
    );
    Ok(reader.duration() as f64 / reader.spec().sample_rate as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    use tempfile::tempdir;

    #[test]
    /// 入力値が正常だったとき
    fn normal() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.wav");
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&input, spec).unwrap();
        let expected = 5;
        for t in (0..44100 * expected).map(|x| x as f32 / 44100.0) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
        writer.finalize().unwrap();
        let result = calculate_wave_seconds(input);
        assert_eq!(result.unwrap(), expected as f64);
        dir.close().unwrap();
    }

    #[test]
    /// 入力値が異常だったとき(wavファイルが存在しない)
    fn error_notfound() {
        let dir = tempdir().unwrap();
        let input = dir.path().join("test.wav");
        let result = calculate_wave_seconds(input);
        assert!(result.is_err());
        dir.close().unwrap();
    }
}
