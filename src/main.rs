use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::process;
use std::time::Duration as StdDuration;

use chrono::Duration;

use crate::softwaretalk::service;

mod config;
mod softwaretalk;
mod srt;
mod text;
mod wav;

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let swtp = service::select_software_talk(&config.softwaretalk_type).unwrap_or_else(|err| {
        eprintln!("Problem selecting software talk: {}", err);
        process::exit(1);
    });
    let files = fs::read_dir(config.dirname).unwrap_or_else(|err| {
        eprintln!("Problem reading directory: {}", err);
        process::exit(1);
    });

    let mut sub_rips = vec![];

    let wavs = files
        .filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"wav")));
    for wav in wavs {
        let serif = swtp.serif_generator(wav.path()).unwrap_or_else(|err| {
            eprintln!("Problem generating serif: {}", err);
            process::exit(1);
        });
        let reader = hound::WavReader::open(wav.path()).unwrap_or_else(|err| {
            eprintln!("Problem opening wav file: {}", err);
            process::exit(1);
        });
        let duration = Duration::from_std(StdDuration::from_secs_f64(wav::calculate_wave_seconds(
            &reader,
        )))
        .unwrap_or_else(|err| {
            eprintln!("Problem calculate wav file seconds: {}", err);
            process::exit(1);
        });
        sub_rips.push(srt::UnitSubRip {
            duration,
            serif: serif.to_string(),
        })
    }
    srt::print_srt(sub_rips);
}
