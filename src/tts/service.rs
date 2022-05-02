//! TTS関係の雑多な関数
use crate::tts::coefont::CoeFont;
use crate::tts::profile;
use crate::tts::softalk::Softalk;
use crate::tts::talqu::Talqu;
use crate::tts::voiceroid::Voiceroid;
use crate::tts::voicevox::Voicevox;

/// profile_nameから対応するTTSを選択する
/// * `profile_name` - TTSの名前
pub fn select_tts_talk(profile_name: &str) -> Result<Box<dyn profile::TTS>, &'static str> {
    info!("input profile_name: {}", profile_name);
    match profile_name {
        "voiceroid" => Ok(Box::new(Voiceroid {})),
        "coefont" => Ok(Box::new(CoeFont {})),
        "voicevox" => Ok(Box::new(Voicevox {})),
        "softalk" => Ok(Box::new(Softalk {})),
        "talqu" => Ok(Box::new(Talqu {})),
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
    // 入力値が正常だったとき(入力値がcoefont)
    fn normal_select_coefont() {
        let input = "coefont";
        let expected = "coefont";
        let result = select_tts_talk(input).unwrap();
        assert_eq!(result.get_profile_name(), expected);
    }

    #[test]
    // 入力値が正常だったとき(入力値がvoicevox)
    fn normal_select_voicevox() {
        let input = "voicevox";
        let expected = "voicevox";
        let result = select_tts_talk(input).unwrap();
        assert_eq!(result.get_profile_name(), expected);
    }

    #[test]
    // 入力値が正常だったとき(入力値がsoftalk)
    fn normal_select_softalk() {
        let input = "softalk";
        let expected = "softalk";
        let result = select_tts_talk(input).unwrap();
        assert_eq!(result.get_profile_name(), expected);
    }

    #[test]
    // 入力値が正常だったとき(入力値がtalqu)
    fn normal_select_talqu() {
        let input = "talqu";
        let expected = "talqu";
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
