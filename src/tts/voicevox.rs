//! VOICEVOX関係のモジュール
use std::error::Error;
use std::path::PathBuf;

use crate::text;
use crate::tts;
use crate::wav;

pub struct Voicevox {}

impl tts::profile::TTS for Voicevox {
    fn serif_generator(&self, path: PathBuf) -> Result<String, Box<dyn Error>> {
        text::generate_subtitle_from_same_name_txt(path)
    }
    fn wave_time_generator(
        &self,
        reader: &hound::WavReader<std::io::BufReader<std::fs::File>>,
    ) -> f64 {
        wav::calculate_wave_seconds(reader)
    }
    fn get_profile_name(&self) -> &'static str {
        "voicevox"
    }
}
