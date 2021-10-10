use std::error::Error;
use std::path::PathBuf;

pub trait TTS {
    fn serif_generator(&self, path: PathBuf) -> Result<String, Box<dyn Error>>;
    fn wave_time_generator(
        &self,
        reader: &hound::WavReader<std::io::BufReader<std::fs::File>>,
    ) -> f64;
    fn get_profile_name(&self) -> &'static str;
}
