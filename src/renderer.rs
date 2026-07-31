use std::io::{self, Write};

use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::Theme;

pub struct Renderer {
    pub theme: Theme,
}

impl Renderer {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }

    pub fn prompt(&self, message: &str) -> io::Result<()> {
        let mut stdout = io::stdout();

        execute!(
            stdout,
            SetForegroundColor(self.theme.pointer),
            Print("? "),
            SetForegroundColor(self.theme.message),
            Print(message),
            Print(": "),
            SetForegroundColor(self.theme.answer),
        )?;

        stdout.flush()
    }

    pub fn clear_prompt(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        execute!(
            stdout,
            MoveUp(1),
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            ResetColor,
        )?;

        stdout.flush()
    }
}
