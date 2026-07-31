use crossterm::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub pointer: Color,
    pub message: Color,
    pub answer: Color,
    pub success: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            pointer: Color::Cyan,
            message: Color::White,
            answer: Color::Green,
            success: Color::Green,
        }
    }
}
