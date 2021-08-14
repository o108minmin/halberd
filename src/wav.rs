pub fn calculate_wave_seconds(reader: &hound::WavReader<std::io::BufReader<std::fs::File>>) -> f64 {
    reader.duration() as f64 / reader.spec().sample_rate as f64
}
