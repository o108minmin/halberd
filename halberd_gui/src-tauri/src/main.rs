#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use halberd_core;
use std::fs;
use env_logger::Builder;
use log::LevelFilter;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn halberd_run(input: &str ,output: &str ,tts: &str) -> String {
    let handle = fs::File::create(output).unwrap();
    let mut config = halberd_core::config::Config {
        tts: tts.to_string(),
        dirname: input.to_string(),
        format: parse_output_extension(output),
        output: handle,
        use_timestamp: true,
    };
    let mut builder = Builder::from_default_env();
    builder.filter_level(LevelFilter::Error).init();

    match halberd_core::run(&mut config) {
        Ok(_) => format!("done"),
        Err(e) => format!("error: {}", e)
    }
}

fn parse_output_extension(output: &str) -> String {
    let v: Vec<&str> = output.split('.').collect();
    return v[v.len() -1].to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![halberd_run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
