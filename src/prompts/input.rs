use crate::{ChevronError, Prompt, Renderer, Validator};
use std::io;

pub struct Input {
    message: String,
    renderer: Renderer,
    validator: Option<Validator<String>>,
}

impl Input {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            renderer: Renderer::new(),
            validator: None,
        }
    }

    pub fn validate<F>(mut self, validator: F) -> Self
    where
        F: Fn(&String) -> Result<(), String> + 'static,
    {
        self.validator = Some(Box::new(validator));
        self
    }
}

impl Prompt for Input {
    type Output = String;

    fn ask(&self) -> Result<Self::Output, ChevronError> {
        loop {
            self.renderer.prompt(&self.message)?;

            let mut value = String::new();
            io::stdin().read_line(&mut value)?;

            let value = value.trim().to_string();

            self.renderer.clear_prompt()?;

            if let Some(validator) = &self.validator {
                if let Err(message) = validator(&value) {
                    self.renderer.error(&message)?;
                    continue;
                }
            }

            return Ok(value);
        }
    }
}
