use crossterm::{
    event::{Event, KeyCode, KeyModifiers, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::{ChevronError, Prompt, Renderer};

pub struct Confirm {
    message: String,
    default: bool,
    renderer: Renderer,
}

impl Confirm {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            default: true,
            renderer: Renderer::new(),
        }
    }

    pub fn default(mut self, value: bool) -> Self {
        self.default = value;
        self
    }
}

impl Prompt for Confirm {
    type Output = bool;

    fn ask(&self) -> Result<bool, ChevronError> {
        self.renderer.prompt(&format!(
            "{} ({})",
            self.message,
            if self.default { "Y/n" } else { "y/N" }
        ))?;

        enable_raw_mode()?;

        let result = loop {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                        break Err(ChevronError::Cancelled);
                    }

                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        break Ok(true);
                    }

                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        break Ok(false);
                    }

                    KeyCode::Enter => {
                        break Ok(self.default);
                    }

                    _ => {}
                }
            }
        };

        disable_raw_mode()?;

        self.renderer.clear_prompt()?;

        result
    }
}
