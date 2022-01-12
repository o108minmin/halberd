//! TTSをモデル化したtrait
use std::error::Error;
use std::path::PathBuf;

pub trait TTS {
    // セリフの生成に使う関数
    fn serif_generator(&self, path: PathBuf) -> Result<String, Box<dyn Error>>;
    // wavファイルの秒数計算に使う関数
    fn wave_time_generator(
        &self,
        reader: &hound::WavReader<std::io::BufReader<std::fs::File>>,
    ) -> f64;
    // TTSの名前
    fn get_profile_name(&self) -> &'static str;
}
