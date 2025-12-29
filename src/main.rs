use inquire::Select;

fn main() {
    let options = vec!["Claude Code", "OpenCode"];

    let answer = Select::new("Choose your development environment:", options).prompt();

    match answer {
        Ok(choice) => {
            println!("\nYou selected: {}", choice);
        }
        Err(_) => {
            println!("Selection cancelled.");
        }
    }
}
