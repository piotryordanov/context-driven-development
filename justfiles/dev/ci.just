# Run full CI checks (fmt, clippy, test)
# Usage: just ci

ci: fmt clippy test
    @echo "✅ All CI checks passed!"
