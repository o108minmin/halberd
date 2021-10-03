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

    #[test]
    fn normal() {
        let input = hound::WavReader::open("tests/data/wav/dummy.wav").unwrap();
        assert!(5.1 >= calculate_wave_seconds(&input) && calculate_wave_seconds(&input) >= 5.0)
    }
}
