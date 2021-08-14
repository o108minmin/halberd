use chrono::prelude::*;
use chrono::Duration;
use chrono::Utc;

pub struct UnitSubRip {
    pub duration: Duration,
    pub serif: String,
}

pub fn print_srt(vec: Vec<UnitSubRip>) {
    let mut cursor = Utc.timestamp(0, 0);
    let mut counter = 1;
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
