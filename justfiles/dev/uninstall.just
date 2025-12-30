# Uninstall the cdd binary from the system
# Usage: just uninstall

uninstall:
    cargo uninstall context-driven-development
    @echo "✅ Uninstalled cdd binary from system"
    @echo "Note: This only removes the binary. CDD files in projects remain."
    @echo "To remove CDD files from a project, run: cdd uninstall"
