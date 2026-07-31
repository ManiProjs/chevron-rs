use chevron::ChevronError;
use chevron::prompts::Input;
use chevron::{Prompt, Theme, theme::set_theme};

use crossterm::style::Color;

fn main() -> Result<(), ChevronError> {
    let theme = Theme {
        pointer: Color::Magenta,
        message: Color::Blue,
        answer: Color::Yellow,
        error: Color::Red,
    };

    set_theme(theme);

    let name = Input::new("Name").ask()?;

    println!("Hello, {name}!");

    Ok(())
}
