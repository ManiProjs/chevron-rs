use chevron::prompts::Input;
use chevron::{Prompt, Theme, theme::set_theme};

use crossterm::style::Color;

fn main() -> std::io::Result<()> {
    let theme = Theme {
        pointer: Color::Magenta,
        message: Color::Blue,
        answer: Color::Yellow,
    };

    set_theme(theme);

    let name = Input::new("Name").ask()?;

    println!("Hello, {name}!");

    Ok(())
}
