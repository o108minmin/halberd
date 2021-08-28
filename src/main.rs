use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::process;
use std::time::Duration as StdDuration;

use chrono::Duration;
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use env_logger::Env;
use env_logger::Builder;
use log::LevelFilter;


use crate::softwaretalk::service;

mod softwaretalk;
mod srt;
mod text;
mod wav;

#[macro_use]
extern crate log;

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
        .arg(
            Arg::new("debug")
                .short('d')
                .about("Print debug level log")
                .required(false)
        )
        .get_matches();
    
    let mut builder = Builder::from_default_env();
    let level = match matches.is_present("debug") {
        false => LevelFilter::Error,
        true => LevelFilter::Debug,
    };
    builder
        .filter_level(level)
        .init();
    info!("start halberd");
    info!("enable debug mode: {}", matches.is_present("debug"));
    info!("input SoftwareTalkType: {}", matches.value_of("SoftwareTalkType").unwrap());
    let swtp = service::select_software_talk(matches.value_of("SoftwareTalkType").unwrap())
        .unwrap_or_else(|err| {
            error!("Problem selecting software talk: {}", err);
            process::exit(1);
        });
    info!("input directory: {}", matches.value_of("Input").unwrap());
    let files = fs::read_dir(matches.value_of("Input").unwrap()).unwrap_or_else(|err| {
        error!("Problem reading directory: {}", err);
        process::exit(1);
    });

    let mut sub_rips = vec![];

    let wavs = files
        .filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"wav")));
    for wav in wavs {
        let serif = swtp.serif_generator(wav.path()).unwrap_or_else(|err| {
            error!("Problem generating serif: {}", err);
            process::exit(1);
        });
        let reader = hound::WavReader::open(wav.path()).unwrap_or_else(|err| {
            error!("Problem opening wav file: {}", err);
            process::exit(1);
        });
        let duration = Duration::from_std(StdDuration::from_secs_f64(wav::calculate_wave_seconds(
            &reader,
        )))
        .unwrap_or_else(|err| {
            error!("Problem calculate wav file seconds: {}", err);
            process::exit(1);
        });
        sub_rips.push(srt::UnitSubRip {
            duration,
            serif: serif.to_string(),
        })
    }
    srt::print_srt(sub_rips);
    info!("end halberd");
}
