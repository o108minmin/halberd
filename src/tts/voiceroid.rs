use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use crate::tts;
use crate::text;
use crate::wav;

pub struct Voiceroid {}

impl tts::profile::TTS for Voiceroid {
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
        "voiceroid"
    }
}

impl fmt::Display for Voiceroid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "voiceroid")
    }
}
