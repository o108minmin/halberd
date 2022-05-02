//! TALQu関係のモジュール
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use crate::text;
use crate::tts;
use crate::wav;

pub struct Talqu {}

impl tts::profile::TTS for Talqu {
    fn serif_generator(&self, path: PathBuf) -> Result<String, Box<dyn Error>> {
        text::generate_subtitle_from_same_name_txt_shift_jis(path)
    }
    fn wave_time_generator(
        &self,
        reader: &hound::WavReader<std::io::BufReader<std::fs::File>>,
    ) -> f64 {
        wav::calculate_wave_seconds(reader)
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
