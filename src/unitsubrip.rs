use std::fmt;

use chrono::Duration;

pub struct UnitSubRip {
    pub duration: Duration,
    pub serif: String,
}

impl fmt::Display for UnitSubRip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "duration: {}, serif: {}", self.duration, self.serif)
    }
}
