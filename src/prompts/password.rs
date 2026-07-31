use std::io;

use crossterm::{
    event::{Event, KeyCode, KeyModifiers, read},
    execute,
    style::Print,
};

use crate::{Prompt, Renderer};

pub struct Password {
    message: String,
    renderer: Renderer,
}

impl Password {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            renderer: Renderer::new(),
        }
    }
}

impl Prompt for Password {
    type Output = String;

    fn ask(&self) -> io::Result<Self::Output> {
        self.renderer.prompt(&self.message)?;

        let _raw_mode = crate::raw_mode::RawMode::new()?;

        let mut password = String::new();
        let mut stdout = io::stdout();

        loop {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Enter => break,

                    KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Interrupted,
                            "cancelled by user",
                        ));
                    }

                    KeyCode::Char(c) => {
                        password.push(c);
                        execute!(stdout, Print("*"))?;
                    }

                    KeyCode::Backspace => {
                        if password.pop().is_some() {
                            execute!(stdout, Print("\u{8} \u{8}"))?;
                        }
                    }

                    _ => {}
                }
            }
        }

        self.renderer.clear_prompt()?;

        Ok(password)
    }
}
