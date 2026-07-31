use chevron::prompts::Password;
use chevron::{ChevronError, Prompt};

fn main() -> Result<(), ChevronError> {
    let password = Password::new("Password").ask()?;

    println!("Length: {}", password.len());

    Ok(())
}
