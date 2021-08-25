use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::process;
use std::time::Duration as StdDuration;

use chrono::Duration;
use clap::{crate_authors, crate_description, crate_version, App, Arg};

use crate::softwaretalk::service;

mod softwaretalk;
mod srt;
mod text;
mod wav;

fn main() {
    let matches = App::new("halberd")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .license("MIT")
        .arg(
            Arg::new("SoftwareTalkType")
                .about("Set a SoftwareTalkType")
                .required(true)
                .possible_values(&["voiceroid", "coefontstudio"])
                .index(1),
        )
        .arg(
            Arg::new("Input")
                .about("input directory")
                .required(false)
                .default_value("./")
                .index(2),
        )
        .get_matches();

    let swtp = service::select_software_talk(matches.value_of("SoftwareTalkType").unwrap())
        .unwrap_or_else(|err| {
            eprintln!("Problem selecting software talk: {}", err);
            process::exit(1);
        });
    let files = fs::read_dir(matches.value_of("Input").unwrap()).unwrap_or_else(|err| {
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
