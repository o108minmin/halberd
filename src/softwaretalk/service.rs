use crate::softwaretalk::coefontstudio::CoeFontStudio;
use crate::softwaretalk::profile;
use crate::softwaretalk::voiceroid::Voiceroid;

pub fn select_software_talk(
    profile_name: &str,
) -> Result<Box<dyn profile::SoftwareTalk>, &'static str> {
    if profile_name == "voiceroid" {
        Ok(Box::new(Voiceroid {}))
    } else if profile_name == "coefontstudio" {
        Ok(Box::new(CoeFontStudio {}))
    } else {
        Err("Didn't match profile name")
    }
}
