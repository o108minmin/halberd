//! cli gateway for halberd.
use clap::{Arg, Command};
use env_logger::Builder;
use log::LevelFilter;
use std::boxed::Box;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process;

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
/// cli gatewayとしてのmain関数
fn main() {
    let matches = Command::new("halberd")
        .bin_name(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .after_long_help(env!("CARGO_PKG_LICENSE"))
        .arg(
            Arg::new("TTS")
                .help("set TTS")
                .required(true)
                .possible_values(&["voiceroid", "coefont", "voicevox", "softalk", "talqu"])
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
            Arg::new("outfile")
                .short('o')
                .long("outfile")
                .help("output file name (if \"stdout\" is defined then using stdout)")
                .default_value("stdout")
                .required(false),
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

    let outfile = matches.value_of("outfile").unwrap().to_string();
    if outfile == "stdout" {
        let out = stdout();
        let handle = out.lock();
        let mut config = config::Config {
            tts: matches.value_of("TTS").unwrap().to_string(),
            dirname: matches.value_of("INPUT").unwrap().to_string(),
            format: matches.value_of("format").unwrap().to_string(),
            output: handle,
            use_ulid: matches.is_present("use-ulid"),
        };
        info!("{:?}", config);
        run(&mut config).unwrap_or_else(|err| {
            error!("Problem running halberd: {}", err);
            process::exit(1);
        });
    } else {
        let handle = fs::File::create(outfile).unwrap_or_else(|err| {
            error!("Problem can't open file: {}", err);
            process::exit(1);
        });
        let mut config = config::Config {
            tts: matches.value_of("TTS").unwrap().to_string(),
            dirname: matches.value_of("INPUT").unwrap().to_string(),
            format: matches.value_of("format").unwrap().to_string(),
            output: handle,
            use_ulid: matches.is_present("use-ulid"),
        };
        info!("{:?}", config);
        run(&mut config).unwrap_or_else(|err| {
            error!("Problem running halberd: {}", err);
            process::exit(1);
        });
    }
}

/// Configを元にhalberdを実行する
pub fn run<W: Write>(config: &mut config::Config<W>) -> Result<(), Box<dyn Error>> {
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
    let mut wavs: Vec<std::path::PathBuf> = Vec::new();
    for entry in dir {
        let path = entry.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "wav" {
                wavs.push(path);
            }
        }
    }
    wavs.sort();

    for f in wavs.iter() {
        let serif = swtp.serif_generator(PathBuf::from(&f))?;
        let reader = hound::WavReader::open(PathBuf::from(&f))?;
        let duration = Duration::seconds_f64(wav::calculate_wave_seconds(&reader));
        sub_rips.push(unitsubrip::UnitSubRip {
            duration,
            serif: serif.to_string(),
        })
    }

    if &config.format == "srt" {
        srt::output_srt(&mut config.output, sub_rips)?;
    } else if &config.format == "xml" {
        xml::output_xml(&mut config.output, sub_rips, config.use_ulid)?;
    }
    info!("end halberd");
    Ok(())
}
