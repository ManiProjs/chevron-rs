#[derive(Clone)]
pub struct Theme {
    pub pointer: &'static str,
    pub checked: &'static str,
    pub unchecked: &'static str,
    pub success: &'static str,
    pub error: &'static str,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            pointer: "❯",
            checked: "◉",
            unchecked: "○",
            success: "✔",
            error: "✘",
        }
    }
}
