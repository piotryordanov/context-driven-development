# Context-Driven Development (CDD) - Just Commands
# A Rust CLI tool for context-driven development workflows

# ============================================================================
# Default Command
# ============================================================================

# Show help message with all available commands
default:
    @just help

# ============================================================================
# Development Commands
# ============================================================================
import 'justfiles/dev/run.just'
import 'justfiles/dev/test.just'
import 'justfiles/dev/fmt.just'
import 'justfiles/dev/clippy.just'
import 'justfiles/dev/check.just'
import 'justfiles/dev/ci.just'
import 'justfiles/dev/dev.just'

# ============================================================================
# Build Commands
# ============================================================================
import 'justfiles/build/build.just'
import 'justfiles/build/clean.just'
import 'justfiles/build/install.just'

# ============================================================================
# Release Commands
# ============================================================================
import 'justfiles/release/version.just'
import 'justfiles/release/pub.just'

# ============================================================================
# Help Command
# ============================================================================

# Show detailed help with command descriptions
help:
    @echo "\033[1;36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\033[0m"
    @echo "\033[1;36m  Context-Driven Development (CDD) - Just Commands\033[0m"
    @echo "\033[1;36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\033[0m"
    @echo ""
    @echo "\033[1;35m🚀 Most Common Commands:\033[0m"
    @echo "  just \033[0;33mrun\033[0m [ARGS]           \033[0;32mRun the CDD CLI with optional arguments\033[0m"
    @echo "  just \033[0;33mtest\033[0m                 \033[0;32mRun all tests\033[0m"
    @echo "  just \033[0;33mbuild\033[0m                \033[0;32mBuild release binary\033[0m"
    @echo "  just \033[0;33minstall\033[0m              \033[0;32mInstall locally (no publish)\033[0m"
    @echo ""
    @echo "\033[1;35m🔧 Development:\033[0m"
    @echo "  just \033[0;33mdev\033[0m                  \033[0;32mFormat code and check compilation\033[0m"
    @echo "  just \033[0;33mfmt\033[0m                  \033[0;32mFormat code with rustfmt\033[0m"
    @echo "  just \033[0;33mclippy\033[0m               \033[0;32mRun clippy linter\033[0m"
    @echo "  just \033[0;33mcheck\033[0m                \033[0;32mCheck code without building\033[0m"
    @echo "  just \033[0;33mci\033[0m                   \033[0;32mRun all CI checks (fmt, clippy, test)\033[0m"
    @echo ""
    @echo "\033[1;35m🏗️  Build:\033[0m"
    @echo "  just \033[0;33mbuild\033[0m                \033[0;32mBuild in release mode\033[0m"
    @echo "  just \033[0;33mclean\033[0m                \033[0;32mClean build artifacts\033[0m"
    @echo "  just \033[0;33minstall\033[0m              \033[0;32mInstall locally without publishing\033[0m"
    @echo ""
    @echo "\033[1;35m📦 Release:\033[0m"
    @echo "  just \033[0;33mversion\033[0m              \033[0;32mShow current version\033[0m"
    @echo "  just \033[0;33mpub\033[0m                  \033[0;32mBump version, publish to crates.io, and install\033[0m"
    @echo ""
    @echo "\033[1;35m📋 Reference:\033[0m"
    @echo "  just \033[0;33m--list\033[0m               \033[0;32mList all available commands\033[0m"
    @echo "  just \033[0;33m--choose\033[0m             \033[0;32mInteractive command picker\033[0m"
    @echo "  just \033[0;33mhelp\033[0m                 \033[0;32mShow this help message\033[0m"
    @echo ""
    @echo "\033[1;36m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\033[0m"
