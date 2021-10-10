use std::fmt::{Display, Formatter, Result};

pub struct Config {
    pub tts: String,
    pub dirname: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Config: tts: {}, dirname: {}",
            self.tts, self.dirname
        )
    }
}
