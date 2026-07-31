use std::io::{self, Write};

use crate::Prompt;

pub struct Input {
    message: String,
}

impl Input {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Prompt for Input {
    type Output = String;

    fn ask(&self) -> io::Result<String> {
        print!("? {}: ", self.message);
        io::stdout().flush()?;

        let mut value = String::new();
        io::stdin().read_line(&mut value)?;

        Ok(value.trim().to_string())
    }
}
