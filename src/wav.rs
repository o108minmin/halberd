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
