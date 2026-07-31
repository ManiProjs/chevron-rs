use std::sync::OnceLock;

use crossterm::style::Color;

static GLOBAL_THEME: OnceLock<Theme> = OnceLock::new();

#[derive(Clone)]
pub struct Theme {
    pub pointer: Color,
    pub message: Color,
    pub answer: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            pointer: Color::Cyan,
            message: Color::White,
            answer: Color::Green,
        }
    }
}

pub fn set_theme(theme: Theme) {
    let _ = GLOBAL_THEME.set(theme);
}

pub fn theme() -> Theme {
    GLOBAL_THEME.get().cloned().unwrap_or_default()
}
