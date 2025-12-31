use include_dir::{include_dir, Dir};
use inquire::Select;
use skim::prelude::*;
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::process;

// Embed the .context/_reference directory at compile time
static REFERENCE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/.context/_reference");

// Get the current package version
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut profile: Option<&str> = None;
    let mut command: Option<String> = None;

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
                        eprintln!("Usage: cdd install --profile <claude|opencode>");
                        process::exit(1);
                    }
                }
                "install" | "setup" => {
                    command = Some("install".to_string());
                    i += 1;
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

    // If install/setup command, run installation
    if command.as_deref() == Some("install") {
        install(profile);
        return;
    }

    // Default: run task selector
    if let Err(e) = run_task() {
        eprintln!("Error running task selector: {}", e);
        process::exit(1);
    }
}

fn install(profile: Option<&str>) {
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

    // Copy command files to profile directory
    if let Err(e) = copy_commands(choice) {
        eprintln!("Error copying commands: {}", e);
        process::exit(1);
    }

    println!("\n✓ Setup complete for {}", choice);
}

fn print_help() {
    println!("cdd (context-driven-development) {}", VERSION);
    println!();
    println!("USAGE:");
    println!("    cdd [COMMAND] [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    (no args)                Fuzzy find and select a task (default)");
    println!("    install, setup           Install/setup CDD in current directory");
    println!("    uninstall, rm, remove    Remove CDD files from current directory");
    println!("    --version, -v            Print version information");
    println!("    --help, -h               Print this help message");
    println!();
    println!("OPTIONS:");
    println!("    -p, --profile <PROFILE>  Specify profile: claude or opencode");
    println!();
    println!("DESCRIPTION:");
    println!("    A tool to help you take your context-driven development to the next level.");
    println!();
    println!("EXAMPLES:");
    println!("    cdd                      # Run task selector (default)");
    println!("    cdd install              # Interactive install - choose profile");
    println!("    cdd install -p opencode  # Install with OpenCode profile");
    println!("    cdd install -p claude    # Install with Claude Code profile");
    println!("    cdd uninstall            # Remove CDD files");
    println!("    cdd --version            # Show version");
}

fn uninstall() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;

    println!("🗑️  Uninstalling CDD files...");

    let mut removed_count = 0;

    // Remove .context directory completely
    let context_path = current_dir.join(".context");
    if context_path.exists() {
        fs::remove_dir_all(&context_path)?;
        println!("  ✓ Removed .context/");
        removed_count += 1;
    }

    // Remove command folders from .claude and .opencode
    let profile_configs = vec![
        (current_dir.join(".claude"), vec!["commands"]),
        (current_dir.join(".opencode"), vec!["command"]),
    ];

    for (profile_dir, folders) in profile_configs {
        if profile_dir.exists() {
            let profile_name = profile_dir.file_name().unwrap().to_string_lossy();

            for folder in folders {
                let folder_path = profile_dir.join(folder);
                if folder_path.exists() {
                    fs::remove_dir_all(&folder_path)?;
                    println!("  ✓ Removed {}/{}/", profile_name, folder);
                    removed_count += 1;
                }
            }

            // Check if profile directory is now empty, if so remove it
            if profile_dir.read_dir()?.next().is_none() {
                fs::remove_dir(&profile_dir)?;
                println!("  ✓ Removed {}/ (was empty)", profile_name);
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

    // Always extract _reference to ensure it's up-to-date
    // This allows users to get latest rules and templates on every run
    println!(
        "📦 Extracting .context/_reference files (version {})...",
        VERSION
    );

    // Create .context directory if it doesn't exist
    fs::create_dir_all(&context_path)?;

    // Extract only _reference from embedded .context (overwrites existing)
    let reference_path = context_path.join("_reference");
    extract_reference_from_embedded(&reference_path)?;

    // Create .context/tasks directory for task files (if doesn't exist)
    let tasks_path = context_path.join("tasks");
    fs::create_dir_all(&tasks_path)?;

    // Write version file
    fs::write(&version_file, VERSION)?;
    println!("✓ Extracted .context/_reference files");

    Ok(())
}

fn extract_reference_from_embedded(target_path: &Path) -> std::io::Result<()> {
    // Extract only rules and templates from _reference, skip commands
    extract_dir_selective(&REFERENCE_DIR, target_path, &["rules", "templates"])?;
    Ok(())
}

fn extract_dir_selective(
    dir: &Dir,
    target_path: &Path,
    allowed_dirs: &[&str],
) -> std::io::Result<()> {
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

    // Recursively extract subdirectories (only if in allowed list)
    for subdir in dir.dirs() {
        // Get just the directory name, not the full path
        let dir_name = subdir.path().file_name().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid directory name")
        })?;

        let dir_name_str = dir_name.to_str().unwrap_or("");

        // Only extract if in allowed list
        if allowed_dirs.contains(&dir_name_str) {
            let subdir_path = target_path.join(dir_name);
            extract_dir_all(subdir, &subdir_path)?;
        }
    }

    Ok(())
}

fn extract_dir_all(dir: &Dir, target_path: &Path) -> std::io::Result<()> {
    // Create the target directory
    fs::create_dir_all(target_path)?;

    // Extract all files
    for file in dir.files() {
        let file_name = file.path().file_name().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file name")
        })?;
        let file_path = target_path.join(file_name);
        fs::write(&file_path, file.contents())?;
    }

    // Recursively extract all subdirectories
    for subdir in dir.dirs() {
        let dir_name = subdir.path().file_name().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid directory name")
        })?;
        let subdir_path = target_path.join(dir_name);
        extract_dir_all(subdir, &subdir_path)?;
    }

    Ok(())
}

fn copy_commands(choice: &str) -> std::io::Result<()> {
    let current_dir = env::current_dir()?;

    // Determine target directory and command folder name
    let (target_dir, command_folder) = match choice {
        "Claude Code" => (current_dir.join(".claude"), "commands"), // plural
        "OpenCode" => (current_dir.join(".opencode"), "command"),   // singular
        _ => unreachable!(),
    };

    // Create target directory if it doesn't exist
    if !target_dir.exists() {
        fs::create_dir(&target_dir)?;
    }

    // Copy commands from embedded REFERENCE_DIR to .claude/commands or .opencode/command
    if let Some(commands_dir) = REFERENCE_DIR.get_dir("commands") {
        let target_commands_dir = target_dir.join(command_folder);
        fs::create_dir_all(&target_commands_dir)?;

        // Copy all command files
        for file in commands_dir.files() {
            let file_name = file.path().file_name().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file name")
            })?;
            let target_path = target_commands_dir.join(file_name);

            // Only copy if file doesn't exist (don't overwrite user's custom commands)
            if !target_path.exists() {
                fs::write(&target_path, file.contents())?;
            }
        }
    }

    Ok(())
}

fn run_task() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let tasks_dir = current_dir.join(".context/tasks");

    // Check if tasks directory exists
    if !tasks_dir.exists() {
        eprintln!("Error: .context/tasks/ directory not found.");
        eprintln!("Run 'cdd' first to initialize the project.");
        process::exit(1);
    }

    // Detect which profile is set up
    let claude_exists = current_dir.join(".claude/commands").exists();
    let opencode_exists = current_dir.join(".opencode/command").exists();

    let (profile_name, command_name) = if opencode_exists {
        ("OpenCode", "opencode")
    } else if claude_exists {
        ("Claude Code", "claude")
    } else {
        eprintln!("Error: Neither .claude/commands nor .opencode/command found.");
        eprintln!("Run 'cdd' first to initialize the project.");
        process::exit(1);
    };

    // Collect all task files
    let mut task_files: Vec<String> = Vec::new();
    for entry in fs::read_dir(&tasks_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            if let Some(file_name) = path.file_name() {
                task_files.push(file_name.to_string_lossy().to_string());
            }
        }
    }

    if task_files.is_empty() {
        println!("No tasks found in .context/tasks/");
        println!("Tasks will appear here after you create them.");
        return Ok(());
    }

    // Sort task files
    task_files.sort();

    // Create input for skim
    let input = task_files.join(
        "
",
    );

    // Get absolute path to tasks directory for preview
    let tasks_dir_abs = tasks_dir.canonicalize().unwrap_or(tasks_dir.clone());
    let task_path_template = tasks_dir_abs.join("{}").display().to_string();

    // Platform-specific preview command
    #[cfg(windows)]
    let preview_cmd = format!("type {}", task_path_template);

    #[cfg(not(windows))]
    let preview_cmd = format!(
        "bat --color=always --style=plain {} 2>/dev/null || cat {}",
        task_path_template, task_path_template
    );

    // Configure skim options with preview
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .preview(Some(&preview_cmd))
        .preview_window(Some("right:60%:wrap"))
        .prompt(Some("Select a task: "))
        .build()
        .unwrap();

    // Run skim
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items));

    match selected_items {
        Some(out) if !out.is_abort => {
            if let Some(item) = out.selected_items.first() {
                let selected_file = item.output().to_string();
                let task_path = tasks_dir.join(&selected_file);

                // Display task information
                println!("\n{}", "=".repeat(80));
                println!("📋 Selected Task: {}", selected_file);
                println!("{}", "=".repeat(80));

                // Read and display task content
                let task_content = match fs::read_to_string(&task_path) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Error reading task file: {}", e);
                        process::exit(1);
                    }
                };

                println!("{}", task_content);
                println!("{}", "=".repeat(80));

                // Launch the appropriate tool
                println!("\n🚀 Launching {}...", profile_name);
                println!("💡 Task file: {}", task_path.display());
                println!();

                let status = process::Command::new(command_name)
                    .current_dir(&current_dir)
                    .status();

                match status {
                    Ok(exit_status) => {
                        if !exit_status.success() {
                            eprintln!(
                                "Warning: {} exited with status: {}",
                                command_name, exit_status
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Error launching {}: {}", command_name, e);
                        eprintln!("Make sure {} is installed and in your PATH.", command_name);
                        process::exit(1);
                    }
                }
            }
        }
        _ => {
            println!("Selection cancelled.");
        }
    }

    Ok(())
}
