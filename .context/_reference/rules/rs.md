# Rust File and Directory Naming

## File Naming

1. **One item per file** - Each function, component, struct, enum, type, or const gets its own file
2. **Filename matches item name** - Use snake_case (e.g., `PlatformInfo` → `platform_info.rs`, `AuthError` → `auth_error.rs`)
3. **Function names include context** - `render_navigation_tree_item` not just `render_item`

## Directory Structure

4. **`main.rs` is minimal** - Only contains `main()` function and module imports
5. **Group files by purpose** - Use directories like `components/`, `services/`, `database/`, `api/`, `types/`
6. **Frontend components go in `src/components/`** - All UI components live here
7. **Structs and complex enums get their own directory** - Structs always use directory structure; enums only when they have multiple impl blocks, methods, or trait implementations
8. **Use `mod.rs` for re-exports** - Each directory has a `mod.rs` that exports its contents

## Crate Names

9. **Cargo.toml uses kebab-case** - Package names use hyphens: `nexus-implement-cli`
10. **Code uses snake_case** - Import with underscores: `use nexus_implement_cli::discovery;`
11. **Auto-conversion** - Cargo automatically converts hyphens to underscores in code
# Rust Type Organization

## Type Organization

1. **Type definition in `mod.rs`** - Only the type definition (struct or complex enum), no impl blocks
2. **`methods/`** - Instance methods that do work (`&self`, `&mut self`)
   - Each file contains its own `#[cfg(test)] mod tests` block
3. **`constructors/`** - Constructors and builders (`new`, `builder`, `with_*`)
   - Each file contains its own `#[cfg(test)] mod tests` block
4. **`traits/`** - Standard library and custom trait implementations
   - Each file contains its own `#[cfg(test)] mod tests` block
5. **NO EXCEPTIONS for structs** - Even simple data holders with 1-2 methods must follow this structure
6. **Complex enums only** - Enums with multiple impl blocks, methods, or traits use this structure; simple enums use single file

## Implementation Style

7. **Each file = one impl block** - Single responsibility per file
8. **Standalone functions where possible** - `render_navigation_tree_item(item)` not `self.render_item(item)`
9. **Impl methods call standalone functions** - Keep `impl` thin, delegate to file functions

## When to Use Directory Structure

**ALWAYS use directory structure for:**
- ALL structs (even pure data holders with only fields and derives)
- Complex enums with multiple impl blocks, methods, or trait implementations

**Use single file for:**
- Standalone functions (not in an impl block)
- Simple enums (enums with only variants, no impl blocks or minimal implementation)
- Type aliases (`type Result<T> = std::result::Result<T, Error>;`)
- Constants (group related constants in files like `src/constants.rs` or `src/config/constants.rs`)

**Never put impl blocks in struct mod.rs file, even for:**
- Simple data holders
- DTOs (Data Transfer Objects)
- Types with only derives and no custom implementations
- Types with only 1-2 methods
- "It seems like overkill" cases

## Example Structure

```
src/components/
├── mod.rs
└── navigation_tree/
    ├── mod.rs                          # Struct definition only
    │
    ├── methods/
    │   ├── mod.rs                      # Re-exports all methods
    │   ├── render_navigation_tree.rs
    │   ├── render_navigation_tree_item.rs
    │   └── toggle_expanded.rs
    │
    ├── traits/
    │   ├── mod.rs                      # Re-exports all trait impls
    │   ├── display.rs                  # impl Display for NavigationTree
    │   ├── default.rs                  # impl Default for NavigationTree
    │   └── from_vec.rs                 # impl From<Vec<TreeItem>> for NavigationTree
    │
    └── constructors/
        ├── mod.rs                      # Re-exports all constructor functions
        ├── new.rs                      # impl NavigationTree { pub fn new() } + inline tests
        └── with_items.rs               # impl NavigationTree { pub fn with_items() } + inline tests
```

## Example: Even Simple Types

**WRONG:**
```rust
// src/types/config.rs
pub struct Config {
    pub name: String,
}

impl Config {
    pub fn new(name: String) -> Self { ... }
}
```

**CORRECT:**
```
src/types/config/
├── mod.rs              # Just: pub struct Config { pub name: String }
└── constructors/
    ├── mod.rs
    └── new.rs          # impl Config { pub fn new(name: String) -> Self { ... } }
```
# Rust Testing Standards

## Every Function Must Have Tests

1. **Separate Test Files:** Every public function must have corresponding test files in a `tests/` subdirectory
2. **Test Coverage:** Test happy path, edge cases, and error conditions  
3. **Test File Naming:** Use pattern `<function_name>.test_<test_description>.rs`
4. **Test Function Naming:** Prefix with `test_` and describe what is being tested
5. **Use `#[should_panic]` attribute:** When testing panic conditions, use this attribute with expected message
6. **Property-based testing:** Consider using `proptest` or `quickcheck` for testing properties across many inputs
7. **Test fixtures:** Use helper functions or modules for common test setup and teardown
8. **Mocking:** Use trait objects or generic parameters to enable mocking in tests

## Test Directory Structure

### Unit Tests (Function-Level)
9. **`tests/` subdirectory alongside code** - Each module/type directory has its own `tests/` subdirectory
10. **One test file per test case** - Each test gets its own file for discoverability
11. **Naming pattern:** `<function_name>.test_<specific_behavior>.rs`

### Integration Tests
12. **`tests/integration/` at crate root** - Integration tests that cross module boundaries, test multiple components together

### End-to-End Tests
13. **`tests/e2e/` at crate root** - End-to-end tests that test the entire system from user perspective

## Example Structure

```
src/components/button/
├── mod.rs                              # Struct definition
├── methods/
│   ├── mod.rs
│   ├── click.rs                        # Implementation
│   └── tests/                          # Unit tests for methods
│       ├── mod.rs                      # Test module setup
│       ├── click.test_sets_clicked_to_true.rs
│       ├── click.test_idempotent.rs
│       └── click.test_fires_callback.rs
├── constructors/
│   ├── mod.rs
│   ├── new.rs
│   └── tests/
│       ├── mod.rs
│       └── new.test_creates_default_state.rs
└── traits/
    ├── mod.rs
    ├── display.rs
    └── tests/
        ├── mod.rs
        └── display.test_formats_correctly.rs
```

## Example Test File

```rust
// src/components/button/methods/tests/click.test_sets_clicked_to_true.rs

use super::super::*;  // Import from parent's parent (the method module)
use crate::components::button::Button;

#[test]
fn test_sets_clicked_to_true() {
    let mut button = Button::new();
    button.click();
    assert!(button.clicked);
}
```

## Test Module Setup

```rust
// src/components/button/methods/tests/mod.rs

mod click {
    pub mod test_sets_clicked_to_true;
    pub mod test_idempotent;
    pub mod test_fires_callback;
}
```

## Benefits of This Approach

- **Filesystem Discoverability:** Can see all tests via file explorer
- **Easy Navigation:** Jump directly to specific test file
- **Better Organization:** Tests grouped by function/feature
- **Parallel Development:** Multiple developers can work on tests without conflicts
- **Clear Naming:** `click.test_idempotent.rs` is self-documenting
# Rust Documentation Standards

## Documentation Requirements

1. **Doc Comments Required:** Every public item must have documentation
   - Structs and their fields
   - Enums and their variants
   - Functions and methods
   - Traits and their methods
   - Type aliases
   - Constants and statics
   - Modules

## Struct Documentation

2. **Struct-level documentation:**
   - Brief description (first line) - What the struct represents
   - Purpose and use cases
   - Example usage

3. **Field documentation:**
   - Document all public fields
   - Explain the purpose and valid values
   - Note any constraints or invariants

## Enum Documentation

4. **Enum-level documentation:**
   - Brief description of what the enum represents
   - When to use each variant
   - Example usage

5. **Variant documentation:**
   - Document each variant's purpose
   - Explain any data associated with variants

## Function Documentation

6. **Required Sections:**
   - Brief description (first line)
   - `# Arguments` - Describe each parameter
   - `# Returns` - Describe return value
   - `# Errors` - Describe error conditions (if function returns Result)
   - `# Panics` - Describe panic conditions (if function can panic)
   - `# Safety` - Document safety invariants (required for unsafe code)
   - `# Performance` - Document algorithmic complexity for performance-critical code
   - `# Example` - Code example showing usage

## Module Documentation

7. **Module-level documentation:**
   - Add at the top of `mod.rs` files using `//!`
   - Explain the module's purpose
   - List main types and concepts
   - Provide usage examples

## Trait Documentation

8. **Trait documentation:**
   - Describe the trait's purpose
   - Explain when to implement it
   - Document associated types and methods
   - Provide implementation examples
# Rust Error Handling

## Error Type Selection

1. **Use `Result<T, E>` for recoverable errors** - Operations that can fail in expected ways should return Result
2. **Use `Option<T>` for expected absence** - Use for values that may or may not exist, not for error conditions
3. **Use `panic!` for unrecoverable errors** - Only for bugs, invariant violations, or truly exceptional conditions
4. **Use `thiserror` for library errors** - When building libraries, use thiserror to create custom error types with proper Error trait implementations
5. **Use `anyhow` for application errors** - In application code, use anyhow for convenient error handling and context

## Error Handling Patterns

6. **Document error conditions** - All possible errors must be documented in the `# Errors` section
7. **Use the `?` operator** - Prefer `?` for error propagation over explicit match or unwrap
8. **Add context to errors** - Use `.context()` or `.with_context()` to add meaningful context when propagating errors
9. **Never use `unwrap()` or `expect()` in production code** - Only acceptable in tests or when panic is genuinely the correct behavior with clear documentation

## thiserror Patterns

10. **Derive `Error` and `Debug`** - Use `#[derive(Error, Debug)]` on error enums
11. **Use `#[error("...")]` attribute** - Define error messages on each variant
12. **Include context in variants** - Use tuple or named fields to include error context

## anyhow Patterns

13. **Import `anyhow::Result`** - Use as return type for functions that can fail
14. **Use `.context()` for static messages** - Add context with string literals
15. **Use `.with_context()` for dynamic messages** - Add context with closures that format strings
16. **Chain context calls** - Add multiple layers of context as errors propagate up the call stack
# Rust Visibility and Encapsulation

## Visibility Rules

1. **Default to private** - All items should be private unless there's a specific reason to expose them
2. **Use `pub(crate)` for internal APIs** - Items shared across modules within the crate but not part of the public API
3. **Use `pub(super)` for parent module access** - Items that should only be visible to the parent module
4. **Document why items are public** - Public items should have documentation explaining their purpose in the public API
5. **Minimize public API surface** - Keep the public API as small as possible to maintain flexibility for internal changes

## Encapsulation Patterns

6. **Encapsulate implementation details** - Hide internal fields, use accessor methods when external access is needed
7. **Consider builder patterns for complex public types** - Rather than exposing all fields publicly, use builders for construction
8. **Private fields with public accessors** - Prefer accessor methods over public fields for better encapsulation
9. **Avoid exposing all fields publicly** - Public fields lock you into the current structure and prevent future changes
# Rust Naming Conventions

## Crate Names

1. **Cargo.toml uses kebab-case** - Package names use hyphens: `nexus-implement-cli`
2. **Code uses snake_case** - Import with underscores: `use nexus_implement_cli::discovery;`
3. **Auto-conversion** - Cargo automatically converts hyphens to underscores in code

## Type Naming

4. **Structs, Enums, Traits use PascalCase** - `TaskMetadata`, `ParseError`, `Display`
5. **Type parameters use single uppercase letters** - `T`, `E`, `K`, `V` or descriptive PascalCase like `State`

## Function and Variable Naming

6. **Functions use snake_case** - `parse_task_file`, `render_navigation_tree`
7. **Function names include context** - `render_navigation_tree_item` not just `render_item`
8. **Variables use snake_case** - `task_id`, `file_path`, `user_count`

## Constant Naming

9. **Constants use SCREAMING_SNAKE_CASE** - `MAX_BUFFER_SIZE`, `DEFAULT_TIMEOUT`
10. **Static variables use SCREAMING_SNAKE_CASE** - `GLOBAL_CONFIG`, `INSTANCE_COUNT`

## Module Naming

11. **Modules use snake_case** - `mod database`, `mod api_client`, `mod error_handling`
12. **File names match module names** - `database.rs`, `api_client/mod.rs`

## Example

```rust
// Crate name: nexus-implement-cli
use nexus_implement_cli::discovery;

const MAX_TASKS: usize = 100;

pub struct TaskMetadata {
    task_id: String,
}

pub enum ParseError {
    FileNotFound,
    InvalidFormat,
}

pub fn parse_task_file(path: &Path) -> Result<TaskMetadata, ParseError> {
    let file_content = read_file(path)?;
    Ok(TaskMetadata { task_id: extract_id(&file_content) })
}
```
# Dioxus Router Navigation

## Navigation Components

1. **Always use `Link` component for navigation** - Never use raw `<a>` tags for internal navigation
2. **Import Route enum** - Add `use crate::router::Route;` to components that need navigation
3. **Type-safe routing** - Use `Link { to: Route::RouteName {}, ... }` syntax
4. **Avoid `<a href>`** - Raw anchor tags cause full page reloads (white flash) instead of SPA navigation

## Link Component Usage

5. **Syntax:** `Link { to: Route::Home {}, class: "btn", "Link Text" }`
6. **Classes work normally** - Apply CSS classes with the `class` attribute
7. **Children supported** - Link can contain text, icons, or other elements
8. **Browser history works** - Back/forward buttons work automatically

## Why Link Over Anchor Tags

9. **Client-side navigation** - `Link` provides instant SPA navigation without page reload
10. **No white flash** - Avoids full WASM reinitialization between pages
11. **Faster transitions** - No network round-trip or asset reloading
12. **Router integration** - Properly integrates with Dioxus router state and hooks

## Example

```rust
use dioxus::prelude::*;
use crate::router::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            // CORRECT: Using Link component
            Link { to: Route::Home {}, class: "nav-link",
                "Home"
            }
            Link { to: Route::Demo {}, class: "nav-link",
                "Demo"
            }
            
            // WRONG: Don't use raw anchor tags for internal routes
            // a { href: "/", "Home" }  // ❌ Causes full page reload
        }
    }
}
```
