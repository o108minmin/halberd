//! cli gateway for halberd.
use std::boxed::Box;
use std::error::Error;
use std::{fs, fmt};
use std::io::Write;
use std::path::{Path, PathBuf};

use time::Duration;

use crate::tts::service;

pub mod config;
pub mod srt;
pub mod text;
pub mod tts;
pub mod unitsubrip;
pub mod wav;
pub mod xml;

#[macro_use]
extern crate log;

impl Error for HalberdError {}
#[derive(Debug)]
struct HalberdError(String);

impl fmt::Display for HalberdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SrtError: {}", self.0)
    }
}

/// Configを元にhalberdを実行する
pub fn run<W: Write>(config: &mut config::Config<W>) -> Result<(), Box<dyn Error>> {
    info!("start halberd");
    info!("input TTS: {}", &config.tts);
    let swtp = service::select_tts_talk(&config.tts)?;
    info!("input directory: {}", &config.dirname);
    let dir = fs::read_dir(&config.dirname)?;
    info!("format: {}", &config.format);
    let mut sub_rips = vec![];
    let mut txts: Vec<std::path::PathBuf> = Vec::new();
    for entry in dir {
        let path = entry.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "txt" {
                txts.push(path);
            }
        }
    }
    txts.sort();

    for f in txts.iter() {
        let serif = swtp.serif_generator(PathBuf::from(&f))?;
        let mut wav = f.clone();
        wav.set_extension("wav");
        // durationを設定: 失敗しても1で設定する
        let duration =
            Duration::seconds_f64(wav::calculate_wave_seconds(PathBuf::from(&wav)).unwrap_or(1.0));
        sub_rips.push(unitsubrip::UnitSubRip {
            duration,
            serif: serif.to_string(),
        })
    }

    if &config.format == "srt" {
        srt::output_srt(&mut config.output, sub_rips)?;
    } else if &config.format == "xml" {
        // event名生成
        let path = Path::new(&config.dirname);
        let dir_name = path.file_name();
        if dir_name.is_none() {
            return Err(Box::new(HalberdError(
                "Problem converting directory name".into(),
            )));
        }
        let event_name = dir_name.unwrap().to_os_string().into_string().unwrap();
        xml::output_xml(
            &mut config.output,
            sub_rips,
            config.use_timestamp,
            event_name,
        )?;
    }
    info!("end halberd");
    Ok(())
}
