#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use halberd_core::run;
use halberd_core::config::Config;
use std::fs;
use env_logger::Builder;
use log::LevelFilter;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn halberd_run(input: &str ,output: &str ,tts: &str) -> String {
    let handle = fs::File::create(output).unwrap();
    let mut config = Config {
        tts: tts.to_string(),
        dirname: input.to_string(),
        format: parse_output_extension(output),
        output: handle,
        // FIXME: macで動かないので一時的に停止
        use_timestamp: false,
    };
    match run(&mut config) {
        Ok(_) => format!("done input: {}", input),
        Err(e) => format!("error: {}", e)
    }
}

fn parse_output_extension(output: &str) -> String {
    let v: Vec<&str> = output.split('.').collect();
    v[v.len() -1].to_string()
}

fn main() {
    let mut builder = Builder::from_default_env();
    builder.filter_level(LevelFilter::Debug).init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![halberd_run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
