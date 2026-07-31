use chevron::Prompt;
use chevron::prompts::Input;

fn main() -> std::io::Result<()> {
    let name = Input::new("What's your name?").ask()?;

    println!("Hello, {name}!");

    Ok(())
}
