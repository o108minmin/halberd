//! TTSをモデル化したtrait
use std::error::Error;
use std::path::PathBuf;

pub trait TTS {
    // 事前処理
    fn setup(&self, path: PathBuf) -> Result<(), Box<dyn Error>>;
    // セリフの生成に使う関数
    fn serif_generator(&self, path: PathBuf) -> Result<String, Box<dyn Error>>;
    // wavファイルの秒数計算に使う関数
    fn wave_time_generator(&self, path: PathBuf) -> Result<f64, Box<dyn Error>>;
    // TTSの名前
    fn get_profile_name(&self) -> &'static str;
}
