use chevron::prompts::Input;
use chevron::{ChevronError, Prompt};

fn main() -> Result<(), ChevronError> {
    let name = Input::new("What's your name?").ask()?;

    println!("Hello, {name}!");

    Ok(())
}
