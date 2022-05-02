//! Softalk関係のモジュール
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use crate::text;
use crate::tts;
use crate::wav;

pub struct Softalk {}

impl tts::profile::TTS for Softalk {
    fn serif_generator(&self, path: PathBuf) -> Result<String, Box<dyn Error>> {
        text::generate_subtitle_from_same_name_txt_utf_16le(path)
    }
    fn wave_time_generator(
        &self,
        reader: &hound::WavReader<std::io::BufReader<std::fs::File>>,
    ) -> f64 {
        wav::calculate_wave_seconds(reader)
    }
    fn get_profile_name(&self) -> &'static str {
        "softalk"
    }
}

impl fmt::Display for Softalk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "softalk")
    }
}
