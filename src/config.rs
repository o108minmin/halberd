//! config関連のモジュール
use std::fmt::{Display, Formatter, Result};

/// Config
/// * `tts` - Text To Speech の種類
/// * `dirname` - 対象のディレクトリ
pub struct Config {
    pub tts: String,
    pub dirname: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Config: tts: {}, dirname: {}", self.tts, self.dirname)
    }
}
