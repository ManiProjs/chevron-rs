use std::io::{self, Write};

use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::{Prompt, Theme};

pub struct Input {
    message: String,
    theme: Theme,
}

impl Input {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            theme: Theme::default(),
        }
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl Prompt for Input {
    type Output = String;

    fn ask(&self) -> io::Result<Self::Output> {
        let mut stdout = io::stdout();

        execute!(
            stdout,
            SetForegroundColor(self.theme.pointer),
            Print("? "),
            SetForegroundColor(self.theme.message),
            Print(&self.message),
            Print(": "),
            SetForegroundColor(self.theme.answer),
        )?;

        stdout.flush()?;

        let mut value = String::new();
        io::stdin().read_line(&mut value)?;

        execute!(
            stdout,
            MoveUp(1),
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            ResetColor,
        )?;

        stdout.flush()?;

        Ok(value.trim().to_string())
    }
}
