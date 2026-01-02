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
        eprintln!("Run 'cdd install' first to initialize the project.");
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
        eprintln!("Run 'cdd install' first to initialize the project.");
        process::exit(1);
    };

    // Collect all task files and directories
    // Format: (display_name, path, is_directory)
    let mut task_items: Vec<(String, std::path::PathBuf, bool)> = Vec::new();

    // Recursively collect both directories and files
    fn collect_items(
        dir: &std::path::Path,
        base: &std::path::Path,
        items: &mut Vec<(String, std::path::PathBuf, bool)>,
    ) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Add the directory itself as a selectable item
                if let Ok(rel_path) = path.strip_prefix(base) {
                    let display_name = format!("📁 {}/", rel_path.display());
                    items.push((display_name, path.clone(), true));
                }
                // Recurse into subdirectories
                collect_items(&path, base, items)?;
            } else if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                // Add markdown files
                if let Ok(rel_path) = path.strip_prefix(base) {
                    let display_name = rel_path.display().to_string();
                    items.push((display_name, path, false));
                }
            }
        }
        Ok(())
    }

    collect_items(&tasks_dir, &tasks_dir, &mut task_items)?;

    if task_items.is_empty() {
        println!("No tasks found in .context/tasks/");
        println!("Tasks will appear here after you create them.");
        return Ok(());
    }

    // Sort: directories first, then files, both alphabetically
    task_items.sort_by(|a, b| {
        match (a.2, b.2) {
            (true, false) => std::cmp::Ordering::Less, // directories before files
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.0.cmp(&b.0), // alphabetical within same type
        }
    });

    // Create input for skim (just the display names)
    let display_names: Vec<String> = task_items.iter().map(|(name, _, _)| name.clone()).collect();
    let input = display_names.join("\n");

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
                let selected_display_name = item.output().to_string();

                // Find the corresponding item
                let selected_item = task_items
                    .iter()
                    .find(|(name, _, _)| name == &selected_display_name);

                let (task_content, display_info) = if let Some((_, path, is_dir)) = selected_item {
                    if *is_dir {
                        // Directory selected - load ALL tasks in this directory
                        let mut combined_content = String::new();
                        let mut task_count = 0;

                        // Collect all markdown files in this directory recursively
                        fn collect_dir_tasks(
                            dir: &std::path::Path,
                            content: &mut String,
                            count: &mut usize,
                        ) -> std::io::Result<()> {
                            for entry in fs::read_dir(dir)? {
                                let entry = entry?;
                                let entry_path = entry.path();
                                if entry_path.is_dir() {
                                    collect_dir_tasks(&entry_path, content, count)?;
                                } else if entry_path.is_file()
                                    && entry_path.extension().is_some_and(|ext| ext == "md")
                                {
                                    if let Ok(file_content) = fs::read_to_string(&entry_path) {
                                        if *count > 0 {
                                            content.push_str("\n\n---\n\n");
                                        }
                                        content.push_str(&format!(
                                            "## File: {}\n\n{}",
                                            entry_path.display(),
                                            file_content
                                        ));
                                        *count += 1;
                                    }
                                }
                            }
                            Ok(())
                        }

                        if let Err(e) =
                            collect_dir_tasks(path, &mut combined_content, &mut task_count)
                        {
                            eprintln!("Error reading directory tasks: {}", e);
                            process::exit(1);
                        }

                        let info = format!("{} ({} tasks)", selected_display_name, task_count);
                        (combined_content, info)
                    } else {
                        // Single file selected
                        match fs::read_to_string(path) {
                            Ok(content) => (content, selected_display_name.clone()),
                            Err(e) => {
                                eprintln!("Error reading task file: {}", e);
                                process::exit(1);
                            }
                        }
                    }
                } else {
                    eprintln!("Error: Selected item not found");
                    process::exit(1);
                };

                // Create the prompt with task content
                let prompt = format!("I want to work on this task:\n\n{}", task_content);

                // Launch the appropriate tool with prompt
                println!(
                    "\n🚀 Launching {} with task: {}",
                    profile_name, display_info
                );
                println!();

                // Different invocation for opencode vs claude
                let status = if command_name == "opencode" {
                    // Try to use user's last used model from OpenCode
                    let mut cmd = process::Command::new(command_name);
                    cmd.current_dir(&current_dir);

                    if let Some(model) = get_opencode_last_model() {
                        cmd.arg("--model").arg(&model);
                    }

                    cmd.arg("--prompt").arg(&prompt).status()
                } else {
                    // claude just takes the prompt as an argument
                    process::Command::new(command_name)
                        .current_dir(&current_dir)
                        .arg(&prompt)
                        .status()
                };

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

fn get_opencode_last_model() -> Option<String> {
    // OpenCode stores recent models in ~/.local/state/opencode/model.json
    let state_file = dirs::home_dir()?.join(".local/state/opencode/model.json");

    if !state_file.exists() {
        return None;
    }

    // Read and parse the JSON file
    let content = fs::read_to_string(&state_file).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;

    // Get the first item from the "recent" array
    let recent = json.get("recent")?.as_array()?;
    let last_model = recent.first()?;

    // Extract providerID and modelID
    let provider_id = last_model.get("providerID")?.as_str()?;
    let model_id = last_model.get("modelID")?.as_str()?;

    Some(format!("{}/{}", provider_id, model_id))
}
