use include_dir::{include_dir, Dir};
use inquire::Select;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

// Embed the .context/_reference directory at compile time
static REFERENCE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/.context/_reference");

// Get the current package version
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // Check for commands
    let args: Vec<String> = env::args().collect();

    // Parse arguments for profile flag and commands
    let mut profile: Option<&str> = None;

    if args.len() > 1 {
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-p" | "--profile" => {
                    if i + 1 < args.len() {
                        let profile_arg = args[i + 1].to_lowercase();
                        profile = match profile_arg.as_str() {
                            "claude" | "claudecode" | "claude-code" => Some("Claude Code"),
                            "opencode" | "open-code" => Some("OpenCode"),
                            _ => {
                                eprintln!("Error: Unknown profile '{}'", args[i + 1]);
                                eprintln!("Valid profiles: claude, opencode");
                                process::exit(1);
                            }
                        };
                        i += 2;
                    } else {
                        eprintln!("Error: --profile requires a value");
                        eprintln!("Usage: cdd --profile <claude|opencode>");
                        process::exit(1);
                    }
                }
                "uninstall" | "rm" | "remove" => {
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
                    eprintln!("Unknown command: {}", args[i]);
                    eprintln!("Run 'cdd --help' for usage information.");
                    process::exit(1);
                }
            }
        }
    }

    // Get choice - either from profile flag or interactive prompt
    let choice = if let Some(profile_choice) = profile {
        profile_choice
    } else {
        let options = vec!["Claude Code", "OpenCode"];
        match Select::new("Choose your development environment:", options)
            .without_help_message()
            .prompt()
        {
            Ok(choice) => choice,
            Err(_) => {
                eprintln!("Selection cancelled.");
                process::exit(1);
            }
        }
    };

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

fn print_help() {
    println!("cdd (context-driven-development) {}", VERSION);
    println!();
    println!("USAGE:");
    println!("    cdd [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    (no args)                Interactive setup - choose Claude Code or OpenCode");
    println!("    uninstall, rm, remove    Remove CDD files from current directory");
    println!("    --version, -v            Print version information");
    println!("    --help, -h               Print this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    A tool to help you take your context-driven development to the next level.");
    println!();
    println!("EXAMPLES:");
    println!("    cdd                # Interactive setup");
    println!("    cdd uninstall      # Remove CDD files");
    println!("    cdd rm             # Same as uninstall");
    println!("    cdd remove         # Same as uninstall");
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
    // Extract the embedded _reference directory directly
    extract_dir(&REFERENCE_DIR, target_path)?;
    Ok(())
}

fn extract_dir(dir: &Dir, target_path: &Path) -> std::io::Result<()> {
    // Create the target directory
    fs::create_dir_all(target_path)?;

    // Extract all files (only files directly in this directory, not subdirectories)
    for file in dir.files() {
        // Get just the filename, not the full path
        let file_name = file.path().file_name().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file name")
        })?;
        let file_path = target_path.join(file_name);
        fs::write(&file_path, file.contents())?;
    }

    // Recursively extract subdirectories
    for subdir in dir.dirs() {
        // Get just the directory name, not the full path
        let dir_name = subdir.path().file_name().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid directory name")
        })?;
        let subdir_path = target_path.join(dir_name);
        extract_dir(subdir, &subdir_path)?;
    }

    Ok(())
}

fn setup_symlinks(choice: &str) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let reference_dir = current_dir.join(".context/_reference");

    if !reference_dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            ".context/_reference directory not found after extraction.",
        ));
    }

    // Determine target directory
    let target_dir = match choice {
        "Claude Code" => current_dir.join(".claude"),
        "OpenCode" => current_dir.join(".opencode"),
        _ => unreachable!(),
    };

    // Create target directory if it doesn't exist
    if !target_dir.exists() {
        fs::create_dir(&target_dir)?;
    }

    // Copy only rules and templates (NOT commands)
    let folders_to_copy = vec!["rules", "templates"];

    for folder in folders_to_copy {
        let source_folder = reference_dir.join(folder);
        let target_folder = target_dir.join(folder);

        if source_folder.exists() {
            copy_directory_contents(&source_folder, &target_folder)?;
        }
    }

    Ok(())
}

fn copy_directory_contents(source: &Path, target: &Path) -> std::io::Result<()> {
    // Create target directory if it doesn't exist
    if !target.exists() {
        fs::create_dir_all(target)?;
    }

    // Copy all files and subdirectories
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let filename = match source_path.file_name() {
            Some(name) => name,
            None => continue,
        };
        let target_path = target.join(filename);

        if source_path.is_dir() {
            // Recursively copy subdirectory
            copy_directory_contents(&source_path, &target_path)?;
        } else if source_path.is_file() {
            // Only copy if file doesn't exist (don't overwrite user's files)
            if !target_path.exists() {
                fs::copy(&source_path, &target_path)?;
            }
        }
    }

    Ok(())
}
