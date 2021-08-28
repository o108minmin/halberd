use std::fmt;

use chrono::prelude::*;
use chrono::Duration;
use chrono::Utc;

pub struct UnitSubRip {
    pub duration: Duration,
    pub serif: String,
}

impl fmt::Display for UnitSubRip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "duration: {}, serif: {}", self.duration, self.serif)
    }
}

pub fn print_srt(vec: Vec<UnitSubRip>) {
    let mut cursor = Utc.timestamp(0, 0);
    let mut counter = 1;
    info!("Print UnitSubRips");
    debug!("input vec length: {}", vec.len());
    for i in vec.iter() {
        println!("{}", counter);
        println!(
            "{} --> {}",
            cursor.format("%H:%M:%S,%3f"),
            (cursor + i.duration).format("%H:%M:%S,%3f")
        );
        println!("{}", i.serif);
        println!();
        cursor = cursor + i.duration;
        counter += 1;
    }
}
