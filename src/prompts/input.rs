use std::io;

use crate::{Prompt, Renderer};

pub struct Input {
    message: String,
    renderer: Renderer,
}

impl Input {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            renderer: Renderer::new(),
        }
    }

    pub fn theme(mut self) -> Self {
        self.renderer = Renderer::new();
        self
    }
}

impl Prompt for Input {
    type Output = String;

    fn ask(&self) -> io::Result<Self::Output> {
        self.renderer.prompt(&self.message)?;

        let mut value = String::new();
        io::stdin().read_line(&mut value)?;

        self.renderer.clear_prompt()?;

        Ok(value.trim().to_string())
    }
}
