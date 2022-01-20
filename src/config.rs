//! config関連のモジュール

/// Config
/// * `tts` - Text To Speech の種類
/// * `dirname` - 対象のディレクトリ
/// * `format` - 出力フォーマット
/// * `use_ulid` - idとしてulidを利用するかどうか
/// * `output` - 出力先
#[derive(Debug)]
pub struct Config<W> {
    pub tts: String,
    pub dirname: String,
    pub format: String,
    pub use_ulid: bool,
    pub output: W,
}
