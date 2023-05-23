//! TALQu関係のモジュール
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use crate::text;
use crate::tts;
use crate::wav;

pub struct Talqu {}

impl tts::profile::TTS for Talqu {
    fn setup(&self, _path: PathBuf) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn serif_generator(&self, path: PathBuf) -> Result<String, Box<dyn Error>> {
        text::generate_subtitle_from_txt_shift_jis(path)
    }
    fn wave_time_generator(&self, path: PathBuf) -> Result<f64, Box<dyn Error>> {
        wav::calculate_wave_seconds(path)
    }
    fn get_profile_name(&self) -> &'static str {
        "talqu"
    }
}

impl fmt::Display for Talqu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "talqu")
    }
}
