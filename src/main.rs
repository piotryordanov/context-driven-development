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
