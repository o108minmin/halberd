use std::fmt::{Display, Formatter, Result};

pub struct Config {
    pub softwaretalk: String,
    pub dirname: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Config: softwaretalk: {}, dirname: {}",
            self.softwaretalk, self.dirname
        )
    }
}
