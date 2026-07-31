use std::io::{self, Write};

use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    pub fn prompt(&self, message: &str) -> io::Result<()> {
        let theme = crate::theme::theme();

        let mut stdout = io::stdout();

        execute!(
            stdout,
            SetForegroundColor(theme.pointer),
            Print("? "),
            SetForegroundColor(theme.message),
            Print(message),
            Print(": "),
            SetForegroundColor(theme.answer),
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
