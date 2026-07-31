use std::fmt::Display;
use std::io::{self, Write};

use crossterm::{
    cursor::{MoveToColumn, MoveUp},
    event::{Event, KeyCode, KeyModifiers, read},
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use crate::{ChevronError, Prompt, Renderer, raw_mode::RawMode, theme};

pub struct Select<T> {
    message: String,
    items: Vec<T>,
    selected: usize,
    renderer: Renderer,
}

impl<T> Select<T>
where
    T: Display,
{
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            items: Vec::new(),
            selected: 0,
            renderer: Renderer::new(),
        }
    }

    pub fn items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        self.items = items.into_iter().collect();
        self
    }

    pub fn default(mut self, index: usize) -> Self {
        if index < self.items.len() {
            self.selected = index;
        }

        self
    }

    fn draw(&self, selected: usize) -> io::Result<()> {
        let theme = theme();
        let mut stdout = io::stdout();

        for (index, item) in self.items.iter().enumerate() {
            execute!(stdout, MoveToColumn(0),)?;

            if index == selected {
                execute!(
                    stdout,
                    SetForegroundColor(theme.pointer),
                    Print(format!("❯ {}\r\n", item)),
                )?;
            } else {
                execute!(
                    stdout,
                    SetForegroundColor(theme.message),
                    Print(format!("  {}\r\n", item)),
                )?;
            }
        }

        execute!(stdout, ResetColor)?;

        stdout.flush()
    }

    fn clear_items(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        for _ in 0..self.items.len() {
            execute!(
                stdout,
                MoveUp(1),
                MoveToColumn(0),
                Clear(ClearType::CurrentLine),
            )?;
        }

        execute!(stdout, MoveToColumn(0),)?;

        stdout.flush()
    }
}

impl<T> Prompt for Select<T>
where
    T: Display + Clone,
{
    type Output = T;

    fn ask(&self) -> Result<Self::Output, ChevronError> {
        let _raw_mode = RawMode::new()?;

        let mut selected = self.selected;

        self.renderer.prompt(&self.message)?;
        println!();

        self.draw(selected)?;

        loop {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }

                    KeyCode::Down => {
                        if selected + 1 < self.items.len() {
                            selected += 1;
                        }
                    }

                    KeyCode::Enter => {
                        self.clear_items()?;

                        return Ok(self.items[selected].clone());
                    }

                    KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Err(ChevronError::Cancelled);
                    }

                    _ => {}
                }

                self.clear_items()?;
                self.draw(selected)?;
            }
        }
    }
}
