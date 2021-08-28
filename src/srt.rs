use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::io::{stdout, Write};
use std::result::Result;

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

pub fn print_srt(vec: Vec<UnitSubRip>) -> Result<(), Box<dyn Error>> {
    let mut cursor = Utc.timestamp(0, 0);
    let mut counter = 1;
    info!("Print UnitSubRips");
    debug!("input vec length: {}", vec.len());
    let out = stdout();
    let mut handle = out.lock();
    for i in vec.iter() {
        info!("{}", i);
        writeln!(handle, "{}", counter)?;
        writeln!(
            handle,
            "{} --> {}",
            cursor.format("%H:%M:%S,%3f"),
            (cursor + i.duration).format("%H:%M:%S,%3f")
        )?;
        writeln!(handle, "{}", i.serif)?;
        writeln!(handle)?;
        cursor = cursor + i.duration;
        counter += 1;
    }
    handle.flush()?;
    Ok(())
}
