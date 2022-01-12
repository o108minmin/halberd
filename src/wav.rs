//! wavファイル関係のモジュール
/// wavファイルの秒数を計算する
/// * `reader` - 秒数を計算したいhoundのwav reader.
pub fn calculate_wave_seconds(reader: &hound::WavReader<std::io::BufReader<std::fs::File>>) -> f64 {
    debug!("input duration: {}", reader.duration());
    debug!("input sampling rate: {}", reader.spec().sample_rate);
    debug!(
        "return {} / {}",
        reader.duration(),
        reader.spec().sample_rate
    );
    reader.duration() as f64 / reader.spec().sample_rate as f64
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
        for t in (0 .. 44100*expected).map(|x| x as f32 / 44100.0) {
            let sample = (t * 440.0 * 2.0 * PI).sin();
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap();
        }
        writer.finalize().unwrap();
        let input_path = hound::WavReader::open(input.to_str().unwrap()).unwrap();
        let result = calculate_wave_seconds(&input_path);
        assert_eq!(result, expected as f64);
        drop(input);
        dir.close().unwrap();
    }
}
