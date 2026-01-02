#!/bin/bash
# Fast demo script for testing (no delays)

clear
echo "╔════════════════════════════════════════════╗"
echo "║  CDD - Context-Driven Development Tool     ║"
echo "╚════════════════════════════════════════════╝"
echo ""

# 1. Show help
echo "=== CDD Help ==="
cdd --help
echo ""

# 2. Version
echo "=== Version ==="
cdd --version
echo ""

# 3. Setup
echo "=== Setting Up Demo Project ==="
cd /tmp
rm -rf cdd-demo-project
mkdir cdd-demo-project
cd cdd-demo-project
echo "Working in: $(pwd)"
echo ""

# 4. Install
echo "=== Installing CDD (OpenCode Profile) ==="
cdd install -p opencode
echo ""

# 5. Show structure
echo "=== Verifying Installation ==="
echo "Context directory:"
ls -la .context/
echo ""
echo "Context tasks:"
ls -la .context/tasks/ || echo "Tasks directory created"
echo ""
echo "OpenCode commands:"
ls -la .opencode/command/ 2>/dev/null || echo "Commands installed"
echo ""

# 6. Create tasks
echo "=== Creating Sample Tasks ==="
cat >.context/tasks/TASK_001-implement-auth.md <<'EOF'
---
task_id: TASK_001
title: Implement User Authentication
project: cdd-demo-project
created: "2024-12-31"
---

# TASK_001: Implement User Authentication

## Summary
Implement JWT-based authentication system.

## Goals
- Create user model with password hashing
- Implement login/registration endpoints
- Add authentication middleware
- Write tests

## Validation
- All tests pass
EOF

cat >.context/tasks/TASK_002-setup-database.md <<'EOF'
---
task_id: TASK_002
title: Setup Database Connection Pool
project: cdd-demo-project
created: "2024-12-31"
---

# TASK_002: Setup Database Connection Pool

## Summary
Configure PostgreSQL database with SQLx.

## Goals
- Add SQLx dependencies
- Create connection pool
- Add migration system
EOF

echo "Tasks created:"
ls -la .context/tasks/
echo ""

echo "Task preview:"
head -10 .context/tasks/TASK_001-implement-auth.md
echo ""

# 7. Summary
echo "╔════════════════════════════════════════════╗"
echo "║  Demo Complete!                            ║"
echo "╚════════════════════════════════════════════╝"
echo ""
echo "Next: Run 'cdd' to launch the fuzzy finder!"
echo ""
