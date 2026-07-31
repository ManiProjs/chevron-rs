use chevron::prompts::Confirm;
use chevron::{ChevronError, Prompt};

fn main() -> Result<(), ChevronError> {
    let delete = Confirm::new("Delete file?").default(false).ask()?;

    println!("Delete: {delete}");

    Ok(())
}
