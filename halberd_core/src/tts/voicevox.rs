//! VOICEVOX関係のモジュール
use std::error::Error;
use std::path::PathBuf;

use crate::text;
use crate::tts;
use crate::wav;

pub struct Voicevox {}

impl tts::profile::TTS for Voicevox {
    fn setup(&self, _path: PathBuf) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn serif_generator(&self, path: PathBuf) -> Result<String, Box<dyn Error>> {
        text::generate_subtitle_from_txt(path)
    }
    fn wave_time_generator(&self, path: PathBuf) -> Result<f64, Box<dyn Error>> {
        wav::calculate_wave_seconds(path)
    }
    fn get_profile_name(&self) -> &'static str {
        "voicevox"
    }
}
