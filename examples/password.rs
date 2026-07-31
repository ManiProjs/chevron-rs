use chevron::Prompt;
use chevron::prompts::Password;

fn main() -> std::io::Result<()> {
    let password = Password::new("Password").ask()?;

    println!("Length: {}", password.len());

    Ok(())
}
