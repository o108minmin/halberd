use std::path::PathBuf;

use crate::softwaretalk;
use crate::text;
use crate::wav;

pub struct CoeFontStudio {}

impl softwaretalk::profile::SoftwareTalk for CoeFontStudio {
    fn serif_generator(&self, path: PathBuf) -> Result<String, &'static str> {
        text::generate_subtitle_from_same_name_txt(path)
    }
    fn wave_time_generator(
        &self,
        reader: &hound::WavReader<std::io::BufReader<std::fs::File>>,
    ) -> f64 {
        wav::calculate_wave_seconds(reader)
    }
}
