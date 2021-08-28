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
        _ => Err("Didn't match profile name {}"),
    }
}
