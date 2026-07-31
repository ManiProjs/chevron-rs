use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct RawMode;

impl RawMode {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;

        Ok(Self)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}
