use crate::softwaretalk::coefontstudio::CoeFontStudio;
use crate::softwaretalk::profile;
use crate::softwaretalk::voiceroid::Voiceroid;

pub fn select_software_talk(
    profile_name: &str,
) -> Result<Box<dyn profile::SoftwareTalk>, &'static str> {
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
    fn normal_select_voiceroid() {
        let input = "voiceroid";
        let expected = "voiceroid";
        let result = select_software_talk(input).unwrap();
        assert_eq!(result.get_profile_name(), expected);
    }

    #[test]
    fn normal_select_coefontstudio() {
        let input = "coefontstudio";
        let expected = "coefontstudio";
        let result = select_software_talk(input).unwrap();
        assert_eq!(result.get_profile_name(), expected);
    }

    #[test]
    #[should_panic]
    fn error_not_found() {
        // TODO: エラーメッセージを使ったテストにする
        let input = "sample";
        match select_software_talk(input) {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };
    }
}
