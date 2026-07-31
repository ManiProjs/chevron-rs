use chevron::prompts::Input;
use chevron::{ChevronError, Prompt};

fn main() -> Result<(), ChevronError> {
    let username = Input::new("Username")
        .validate(|value| {
            if value.len() < 3 {
                Err("Username must be at least 3 characters".into())
            } else {
                Ok(())
            }
        })
        .ask()?;

    println!("Welcome, {username}");

    Ok(())
}
