//! TTS関係の雑多な関数
use crate::tts::coefontstudio::CoeFontStudio;
use crate::tts::profile;
use crate::tts::voiceroid::Voiceroid;

/// profile_nameから対応するTTSを選択する
/// * `profile_name` - TTSの名前
pub fn select_tts_talk(
    profile_name: &str,
) -> Result<Box<dyn profile::TTS>, &'static str> {
    info!("input profile_name: {}", profile_name);
    match profile_name {
        "voiceroid" => Ok(Box::new(Voiceroid {})),
        "coefontstudio" => Ok(Box::new(CoeFontStudio {})),
        _ => Err("Didn't match profile name"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 入力値が正常だったとき(入力値がvoiceroid)
    fn normal_select_voiceroid() {
        let input = "voiceroid";
        let expected = "voiceroid";
        let result = select_tts_talk(input).unwrap();
        assert_eq!(result.get_profile_name(), expected);
    }

    #[test]
    // 入力値が正常だったとき(入力値がcoefontstudio)
    fn normal_select_coefontstudio() {
        let input = "coefontstudio";
        let expected = "coefontstudio";
        let result = select_tts_talk(input).unwrap();
        assert_eq!(result.get_profile_name(), expected);
    }

    #[test]
    #[should_panic]
    // 入力値が異常だったとき(入力値がどのTTSとも一致しない時)
    fn error_not_found() {
        // TODO: エラーメッセージを使ったテストにする
        let input = "sample";
        match select_tts_talk(input) {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };
    }
}
