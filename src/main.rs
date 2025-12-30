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
    // Check for commands
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "uninstall" => {
                if let Err(e) = uninstall() {
                    eprintln!("Error during uninstall: {}", e);
                    process::exit(1);
                }
                return;
            }
            "--version" | "-v" => {
                println!("cdd (context-driven-development) {}", VERSION);
                return;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            _ => {
                eprintln!("Unknown command: {}", args[1]);
                eprintln!("Run 'cdd --help' for usage information.");
                process::exit(1);
            }
        }
    }

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

fn print_help() {
    println!("cdd (context-driven-development) {}", VERSION);
    println!();
    println!("USAGE:");
    println!("    cdd [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    (no args)          Interactive setup - choose Claude Code or OpenCode");
    println!("    uninstall          Remove CDD files from current directory");
    println!("    --version, -v      Print version information");
    println!("    --help, -h         Print this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    Sets up context-driven development environment by:");
    println!("    - Extracting .context/_reference files");
    println!("    - Copying command files to .claude/commands or .opencode/command");
    println!();
    println!("EXAMPLES:");
    println!("    cdd                # Interactive setup");
    println!("    cdd uninstall      # Remove CDD files");
    println!("    cdd --version      # Show version");
}

fn uninstall() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;

    println!("🗑️  Uninstalling CDD files...");

    // List of paths to remove
    let paths_to_remove = vec![
        current_dir.join(".context"),
        current_dir.join(".claude"),
        current_dir.join(".opencode"),
    ];

    let mut removed_count = 0;

    for path in paths_to_remove {
        if path.exists() {
            let path_name = path.file_name().unwrap().to_string_lossy();
            if path.is_dir() {
                fs::remove_dir_all(&path)?;
                println!("  ✓ Removed {}/", path_name);
                removed_count += 1;
            } else {
                fs::remove_file(&path)?;
                println!("  ✓ Removed {}", path_name);
                removed_count += 1;
            }
        }
    }

    if removed_count == 0 {
        println!("  No CDD files found to remove.");
    } else {
        println!(
            "\n✅ Uninstall complete! Removed {} item(s).",
            removed_count
        );
        println!("Note: This only removed CDD files from the current directory.");
        println!(
            "To uninstall the cdd binary itself, run: cargo uninstall context-driven-development"
        );
    }

    Ok(())
}

fn ensure_context_extracted() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let context_path = current_dir.join(".context");
    let version_file = context_path.join(".version");

    // Check if we need to extract/update
    let needs_extraction = if !version_file.exists() {
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
        println!(
            "📦 Extracting .context/_reference files (version {})...",
            VERSION
        );

        // Create .context directory if it doesn't exist
        fs::create_dir_all(&context_path)?;

        // Extract only _reference from embedded .context
        let reference_path = context_path.join("_reference");
        extract_reference_from_embedded(&reference_path)?;

        // Write version file
        fs::write(&version_file, VERSION)?;
        println!("✓ Extracted .context/_reference files");
    }

    Ok(())
}

fn extract_reference_from_embedded(target_path: &Path) -> std::io::Result<()> {
    // Find _reference in the embedded CONTEXT_DIR
    if let Some(reference_dir) = CONTEXT_DIR.get_dir("_reference") {
        extract_dir(reference_dir, target_path)?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "_reference directory not found in embedded .context",
        ));
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

    // Determine target directory and folder name for commands
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
