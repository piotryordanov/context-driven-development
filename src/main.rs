use inquire::Select;
use std::env;
use std::fs;
use std::process;

fn main() {
    let options = vec!["Claude Code", "OpenCode"];

    let answer = Select::new("Choose your development environment:", options)
        .with_help_message("")
        .prompt();

    match answer {
        Ok(choice) => {
            if let Err(e) = setup_symlinks(choice) {
                eprintln!("Error setting up symlinks: {}", e);
                process::exit(1);
            }
            println!("\n✓ Setup complete for {}", choice);
        }
        Err(_) => {
            eprintln!("Selection cancelled.");
            process::exit(1);
        }
    }
}

fn setup_symlinks(choice: &str) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let context_commands = current_dir.join(".context/commands");

    if !context_commands.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            ".context/commands directory not found. Make sure you're running this from the project root."
        ));
    }

    let target = match choice {
        "Claude Code" => current_dir.join(".claude"),
        "OpenCode" => current_dir.join(".opencode"),
        _ => unreachable!(),
    };

    // Remove existing symlink/directory if it exists
    if target.exists() || target.read_link().is_ok() {
        if target.is_dir() && target.read_link().is_err() {
            // It's a real directory, not a symlink - don't remove it
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("{} already exists as a directory. Please remove it manually if you want to replace it with a symlink.", target.display())
            ));
        } else {
            // It's a symlink or broken symlink, remove it
            let _ = fs::remove_file(&target);
        }
    }

    // Create symlink
    #[cfg(unix)]
    std::os::unix::fs::symlink(&context_commands, &target)?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&context_commands, &target)?;

    Ok(())
}
