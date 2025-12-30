use include_dir::{include_dir, Dir};
use inquire::Select;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

// Embed the .context directory at compile time
static CONTEXT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/.context");

// Get the current package version
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let options = vec!["Claude Code", "OpenCode"];

    let answer = Select::new("Choose your development environment:", options)
        .with_help_message("")
        .prompt();

    match answer {
        Ok(choice) => {
            // Ensure .context is extracted and up-to-date
            if let Err(e) = ensure_context_extracted() {
                eprintln!("Error setting up .context: {}", e);
                process::exit(1);
            }

            // Create symlinks
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

fn ensure_context_extracted() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let context_path = current_dir.join(".context");
    let version_file = context_path.join(".version");

    // Check if we need to extract/update
    let needs_extraction = if !context_path.exists() || !version_file.exists() {
        true // First time or no version file
    } else {
        // Compare versions
        let existing_version = fs::read_to_string(&version_file)
            .unwrap_or_default()
            .trim()
            .to_string();
        existing_version != VERSION
    };

    if needs_extraction {
        println!("📦 Extracting .context files (version {})...", VERSION);
        extract_dir(&CONTEXT_DIR, &context_path)?;
        fs::write(&version_file, VERSION)?;
        println!("✓ Extracted .context files");
    }

    Ok(())
}

fn extract_dir(dir: &Dir, target_path: &Path) -> std::io::Result<()> {
    // Create the target directory
    fs::create_dir_all(target_path)?;

    // Extract all files
    for file in dir.files() {
        let file_path = target_path.join(file.path());
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file_path, file.contents())?;
    }

    // Recursively extract subdirectories
    for subdir in dir.dirs() {
        let subdir_path = target_path.join(subdir.path());
        extract_dir(subdir, &subdir_path)?;
    }

    Ok(())
}

fn setup_symlinks(choice: &str) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let context_commands = current_dir.join(".context/_reference/commands");

    if !context_commands.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            ".context/_reference/commands directory not found after extraction.",
        ));
    }

    // Determine target directory and folder name
    let (target_dir, folder_name) = match choice {
        "Claude Code" => (current_dir.join(".claude"), "commands"), // plural for Claude
        "OpenCode" => (current_dir.join(".opencode"), "command"),   // singular for OpenCode
        _ => unreachable!(),
    };

    // Create target directory if it doesn't exist
    if !target_dir.exists() {
        fs::create_dir(&target_dir)?;
    }

    // Create the commands/command folder
    let target_commands_dir = target_dir.join(folder_name);
    if !target_commands_dir.exists() {
        fs::create_dir(&target_commands_dir)?;
    }

    // Copy all command files from .context/_reference/commands to target
    for entry in fs::read_dir(&context_commands)? {
        let entry = entry?;
        let source_path = entry.path();

        // Skip if not a file
        if !source_path.is_file() {
            continue;
        }

        // Get the filename
        let filename = match source_path.file_name() {
            Some(name) => name,
            None => continue,
        };

        // Copy to target directory
        let target_path = target_commands_dir.join(filename);

        // Only copy if file doesn't exist (don't overwrite user's custom commands)
        if !target_path.exists() {
            fs::copy(&source_path, &target_path)?;
        }
    }

    Ok(())
}
