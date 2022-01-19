//! cli gateway for halberd.
use clap::{App, Arg};
use env_logger::Builder;
use log::LevelFilter;
use std::boxed::Box;
use std::env;
use std::error::Error;
use std::fs;
use std::io::stdout;
use std::path::PathBuf;
use std::process;
use std::time::Duration as StdDuration;

use chrono::Duration;

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
/// cli gatewayとしてのmain関数
fn main() {
    let matches = App::new("halberd")
        .bin_name(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_long_help(env!("CARGO_PKG_LICENSE"))
        .arg(
            Arg::new("TTS")
                .help("set TTS")
                .required(true)
                .possible_values(&["voiceroid", "coefont"])
                .index(1),
        )
        .arg(
            Arg::new("INPUT")
                .help("input directory")
                .required(false)
                .default_value("./")
                .index(2),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("output format")
                .required(false)
                .default_value("srt")
                .possible_values(&["srt", "xml"]),
        )
        .arg(
            Arg::new("use-ulid")
                .short('u')
                .long("use-ulid")
                .help("using ulid for some idenfities")
                .required(false),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Print debug level log")
                .required(false),
        )
        .get_matches();

    let mut builder = Builder::from_default_env();
    let level = match matches.is_present("debug") {
        false => LevelFilter::Error,
        true => LevelFilter::Debug,
    };
    builder.filter_level(level).init();
    info!("create halberd config");
    info!("enable debug mode: {}", matches.is_present("debug"));
    info!("build config");
    let config = config::Config {
        tts: matches.value_of("TTS").unwrap().to_string(),
        dirname: matches.value_of("INPUT").unwrap().to_string(),
        format: matches.value_of("format").unwrap().to_string(),
        use_ulid: matches.is_present("use-ulid"),
    };
    info!("{}", config);
    run(config).unwrap_or_else(|err| {
        error!("Problem running halberd: {}", err);
        process::exit(1);
    });
}

/// Configを元にhalberdを実行する
pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    info!("start halberd");
    info!("input TTS: {}", &config.tts);
    let swtp = service::select_tts_talk(&config.tts).unwrap_or_else(|err| {
        error!("Problem selecting tts talk: {}", err);
        process::exit(1);
    });
    info!("input directory: {}", &config.dirname);
    let dir = fs::read_dir(&config.dirname).unwrap_or_else(|err| {
        error!("Problem reading directory: {}", err);
        process::exit(1);
    });
    info!("format: {}", &config.format);
    let mut sub_rips = vec![];

    let wavs = dir
        .filter_map(Result::ok)
        .filter(|d| d.path().extension().unwrap() == "wav")
        .collect::<Vec<_>>();

    let mut file_names = vec![];
    for w in wavs {
        let tmp = w.path();
        file_names.push(tmp);
    }
    file_names.sort();

    for f in file_names.iter() {
        let serif = swtp.serif_generator(PathBuf::from(&f))?;
        let reader = hound::WavReader::open(PathBuf::from(&f))?;
        let duration = Duration::from_std(StdDuration::from_secs_f64(
            wav::calculate_wave_seconds(&reader),
        ))?;
        sub_rips.push(unitsubrip::UnitSubRip {
            duration,
            serif: serif.to_string(),
        })
    }
    let out = stdout();
    let mut handle = out.lock();
    if &config.format == "srt" {
        srt::output_srt(&mut handle, sub_rips)?;
    } else if &config.format == "xml" {
        xml::output_xml(&mut handle, sub_rips, config.use_ulid)?;
    }
    info!("end halberd");
    Ok(())
}
