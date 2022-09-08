use clap::{Arg, Command};
use env_logger::Builder;
use log::LevelFilter;
use std::fs;
use std::io::stdout;
use std::process;
use std::path::Path;
use core::run;
use core::config;



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
                .help("output file name (if \"stdout\" is defined then using stdout) (if \"dirname\" is defined then using dirname)")
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
            Arg::new("use-timestamp")
                .short('t')
                .long("use-timestamp")
                .help("using timestamp for the event name(xml only)")
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

    let mut outfile = matches.value_of("outfile").unwrap().to_string();
    if outfile == "stdout" {
        let out = stdout();
        let handle = out.lock();
        let mut config = config::Config {
            tts: matches.value_of("TTS").unwrap().to_string(),
            dirname: matches.value_of("INPUT").unwrap().to_string(),
            format: matches.value_of("format").unwrap().to_string(),
            output: handle,
            use_timestamp: matches.is_present("use-timestamp"),
        };
        info!("{:?}", config);
        run(&mut config).unwrap_or_else(|err| {
            error!("Problem running halberd: {}", err);
            process::exit(1);
        });
    } else {
        if outfile == "dirname" {
            // ファイル名にディレクトリ名を使用する指定があった場合
            let fullpath = matches.value_of("INPUT").unwrap().to_string();
            let path = Path::new(&fullpath);
            let dir_name = match path.file_name() {
                None => panic!("Problem converting directory name"),
                Some(s) => s,
            };
            let format = matches.value_of("format").unwrap().to_string();
            outfile = dir_name.to_os_string().into_string().unwrap() + "." + &format;
        }
        let handle = fs::File::create(outfile).unwrap_or_else(|err| {
            error!("Problem can't open file: {}", err);
            process::exit(1);
        });
        let mut config = config::Config {
            tts: matches.value_of("TTS").unwrap().to_string(),
            dirname: matches.value_of("INPUT").unwrap().to_string(),
            format: matches.value_of("format").unwrap().to_string(),
            output: handle,
            use_timestamp: matches.is_present("use-timestamp"),
        };
        info!("{:?}", config);
        run(&mut config).unwrap_or_else(|err| {
            error!("Problem running halberd: {}", err);
            process::exit(1);
        });
    }
}
