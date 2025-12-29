# Context-Driven Development (CDD)

A CLI tool for context-driven development workflows.

## Installation

Install from crates.io:

```bash
cargo install context-driven-development
```

Or install directly from GitHub:

```bash
cargo install --git https://github.com/piotryordanov/context-driven-development
```

### Using cargo-binstall (faster)

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installed:

```bash
cargo binstall context-driven-development
```

### Using cargox (npx-like)

If you have [cargox](https://github.com/pkgxdev/cargox) installed:

```bash
cargox context-driven-development
```

## Usage

Run the CLI with either command:

```bash
cdd
# or
context-driven-development
```

The tool will present an interactive menu to choose your development environment.

## Development

### Prerequisites

- Rust 1.70.0 or later
- [just](https://github.com/casey/just) command runner (optional, but recommended)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/piotryordanov/context-driven-development
cd context-driven-development

# Install locally
cargo install --path .

# Or use just
just install
```

### Available Commands

If you have `just` installed:

```bash
# Show all available commands
just

# Development
just run          # Run the CLI
just test         # Run tests
just dev          # Format and check code
just ci           # Run CI checks (fmt, clippy, test)

# Build
just build        # Build release binary
just clean        # Clean build artifacts
just install      # Install locally

# Release
just version      # Show current version
just pub          # Bump version, publish, and install
```

### Project Structure

```
.
├── src/
│   └── main.rs           # Main CLI application
├── justfiles/            # Just command recipes
│   ├── build/            # Build-related commands
│   ├── dev/              # Development commands
│   └── release/          # Release commands
├── Cargo.toml
└── justfile              # Main justfile
```

## Binary Names

This package provides two binaries with the same functionality:

- `context-driven-development` - Full name
- `cdd` - Short alias

Use whichever you prefer!

## License

MIT

## Repository

https://github.com/piotryordanov/context-driven-development
