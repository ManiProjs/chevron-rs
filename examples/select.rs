use chevron::prompts::Select;
use chevron::{ChevronError, Prompt};

fn main() -> Result<(), ChevronError> {
    let web_framework = Select::new("Web framework")
        .items(["Axum", "Django", "Ruby on Rails"])
        .default(1)
        .ask()?;

    println!("Selected: {web_framework}");

    Ok(())
}
