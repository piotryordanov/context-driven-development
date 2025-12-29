#!/usr/bin/env bash

# Find the project root by locating .context directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Verify .context exists at project root
if [ ! -d "$PROJECT_ROOT/.context" ]; then
	echo "Error: .context directory not found at $PROJECT_ROOT" >&2
	exit 1
fi

# Change to project root
cd "$PROJECT_ROOT" || exit 1

# Print working directory
echo "Working directory: $(pwd)"
