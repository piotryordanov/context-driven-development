# Run the project with optional arguments
# Usage: just run [ARGS...]
# Example: just run --help

run *ARGS:
    cargo run -- {{ARGS}}
