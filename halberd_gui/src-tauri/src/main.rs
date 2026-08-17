#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use env_logger::Builder;
use halberd_core::config::Config;
use halberd_core::run;
use log::LevelFilter;
use std::fs;
use std::path::Path;

// Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/
#[tauri::command]
fn halberd_run(input: &str, output: &str, tts: &str) -> Result<String, String> {
    let handle = fs::File::create(output)
        .map_err(|error| format!("出力ファイルを作成できませんでした: {error}"))?;
    let format = Path::new(output)
        .extension()
        .and_then(|extension| extension.to_str())
        .ok_or_else(|| "出力ファイルの形式を判定できませんでした".to_string())?;
    let mut config = Config {
        tts: tts.to_string(),
        dirname: input.to_string(),
        format: format.to_string(),
        output: handle,
        // FIXME: macで動かないので一時的に停止
        use_timestamp: false,
    };
    run(&mut config).map_err(|error| error.to_string())?;
    Ok(format!("保存しました: {output}"))
}

fn main() {
    let mut builder = Builder::from_default_env();
    builder.filter_level(LevelFilter::Debug).init();
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![halberd_run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
