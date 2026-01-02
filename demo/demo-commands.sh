#!/bin/bash
# Demo script for CDD - Context-Driven Development Tool
# This script demonstrates the key features of cdd

# Add delay between commands for readability
command_delay=2
output_delay=1.5

# Helper to show what we're doing
demo_header() {
	echo ""
	echo "=============================================="
	echo "$1"
	echo "=============================================="
	echo ""
	sleep $command_delay
}

# Helper to run commands with delay
run_cmd() {
	echo "$ $1"
	sleep 0.5
	eval "$1"
	sleep $output_delay
}

# Start
clear
echo "╔════════════════════════════════════════════╗"
echo "║  CDD - Context-Driven Development Tool     ║"
echo "║  Demo Recording                            ║"
echo "╚════════════════════════════════════════════╝"
sleep $command_delay

# 1. Show help
demo_header "1. CDD Help - Available Commands"
run_cmd "cdd --help"

# 2. Show version
demo_header "2. Check Version"
run_cmd "cdd --version"

# 3. Set up demo project
demo_header "3. Setting Up Demo Project"
run_cmd "cd /tmp"
run_cmd "rm -rf cdd-demo-project"
run_cmd "mkdir cdd-demo-project"
run_cmd "cd cdd-demo-project"
run_cmd "pwd"

# 4. Install CDD with OpenCode profile
demo_header "4. Install CDD (OpenCode Profile)"
run_cmd "cdd install -p opencode"

# 5. Show created structure
demo_header "5. Verify Installation - Context Directory"
run_cmd "ls -la .context/"
sleep 1
run_cmd "tree -L 2 .context/ 2>/dev/null || find .context -type f | head -10"

demo_header "6. Verify Installation - OpenCode Commands"
run_cmd "ls -la .opencode/command/ 2>/dev/null || ls -la .opencode/"

# 6. Create a sample task
demo_header "7. Create a Sample Task"
cat >.context/tasks/TASK_001-implement-auth.md <<'EOF'
---
task_id: TASK_001
title: Implement User Authentication
project: cdd-demo-project
created: "2024-12-31"
---

FILE NAMING: TASK_001-implement-auth.md

# TASK_001: Implement User Authentication

## Summary

Implement JWT-based authentication system with login and registration endpoints.

## Goals

- Create user model with password hashing
- Implement login endpoint with JWT token generation
- Implement registration endpoint with validation
- Add authentication middleware
- Write integration tests

## File System Diff

```
+ src/models/user.rs
+ src/auth/jwt.rs
+ src/auth/middleware.rs
+ src/handlers/auth.rs
+ tests/integration/auth_test.rs
```

## Validation

- `cargo test` - All tests pass
- `cargo clippy` - No warnings
- Manual testing with curl/Postman
EOF

run_cmd "cat .context/tasks/TASK_001-implement-auth.md"

# 7. Create another task
demo_header "8. Create Another Task"
cat >.context/tasks/TASK_002-setup-database.md <<'EOF'
---
task_id: TASK_002
title: Setup Database Connection Pool
project: cdd-demo-project
created: "2024-12-31"
---

FILE NAMING: TASK_002-setup-database.md

# TASK_002: Setup Database Connection Pool

## Summary

Configure PostgreSQL database with connection pooling using SQLx.

## Goals

- Add SQLx dependencies
- Create database configuration
- Implement connection pool
- Add migration system
- Create initial schema

## Validation

- Database connection successful
- Migrations run without errors
EOF

run_cmd "ls -la .context/tasks/"

# 8. Show the task selection interface info
demo_header "9. Task Selection with CDD"
echo "Now you would normally run 'cdd' (no arguments) to:"
echo ""
echo "  ✓ See a fuzzy finder with all your tasks"
echo "  ✓ Preview task content in real-time"
echo "  ✓ Select a task and launch OpenCode with full context"
echo ""
echo "The fuzzy finder interface would look like this:"
echo ""
echo "  > TASK_001-implement-auth.md"
echo "    TASK_002-setup-database.md"
echo ""
echo "  2/2"
echo "  >"
echo ""
sleep 3

# 9. Uninstall demo
demo_header "10. Cleanup (Optional)"
echo "To remove CDD from a project:"
echo "$ cdd uninstall"
echo ""
echo "(We'll skip this in the demo to preserve our setup)"
sleep 2

# Final
echo ""
echo "╔════════════════════════════════════════════╗"
echo "║  Demo Complete!                            ║"
echo "╚════════════════════════════════════════════╝"
echo ""
echo "Key Takeaways:"
echo "  • 'cdd install' sets up context-driven development"
echo "  • Tasks stored in .context/tasks/"
echo "  • 'cdd' launches fuzzy finder for task selection"
echo "  • Integrates with OpenCode/Claude Code"
echo ""
echo "Learn more: https://github.com/yourusername/cdd"
echo ""
sleep 2
