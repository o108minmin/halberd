//! TTSをモデル化したtrait
use std::error::Error;
use std::path::PathBuf;

pub trait TTS {
    /// 事前処理を実行します。
    ///
    /// # Errors
    ///
    /// TTS固有の初期化に失敗した場合にエラーを返します。
    fn setup(&self, path: PathBuf) -> Result<(), Box<dyn Error>>;

    /// 指定された音声ファイルに対応するセリフを生成します。
    ///
    /// # Errors
    ///
    /// TTS固有のセリフ生成に失敗した場合にエラーを返します。
    fn serif_generator(&self, path: PathBuf) -> Result<String, Box<dyn Error>>;

    /// 指定された音声ファイルの再生時間を取得します。
    ///
    /// # Errors
    ///
    /// 音声ファイルの再生時間を取得できない場合にエラーを返します。
    fn wave_time_generator(&self, path: PathBuf) -> Result<f64, Box<dyn Error>>;

    /// TTSの名前を返します。
    fn get_profile_name(&self) -> &'static str;
}
