# VHS tape file for CDD demo
# See: https://github.com/charmbracelet/vhs

Output demo/cdd-demo.gif

Set FontSize 22
Set Width 1200
Set Height 800
Set Theme "Dracula"
Set TypingSpeed 40ms
Set PlaybackSpeed 1.0

# Clear and show title
Type "clear && echo '╔════════════════════════════════════════╗'"
Enter
Type "echo '║   CDD - Context-Driven Development   ║'"
Enter
Type "echo '╚════════════════════════════════════════╝'"
Enter
Sleep 1s

# Show help
Type "cdd --help"
Enter
Sleep 2s

# Setup demo
Type "cd /tmp && rm -rf cdd-demo && mkdir cdd-demo && cd cdd-demo"
Enter
Sleep 500ms

# Install
Type "cdd install -p opencode"
Enter
Sleep 1.5s

# Show structure
Type "ls .context/"
Enter
Sleep 1s

Type "ls .opencode/command/"
Enter  
Sleep 1s

# Create task (simplified)
Type "echo '---' > .context/tasks/TASK_001-demo.md"
Enter
Type "echo 'task_id: TASK_001' >> .context/tasks/TASK_001-demo.md"
Enter
Type "echo '---' >> .context/tasks/TASK_001-demo.md"
Enter
Sleep 500ms

# Show task
Type "cat .context/tasks/TASK_001-demo.md"
Enter
Sleep 1.5s

# Final
Type "echo '' && echo 'Run: cdd' && echo ''"
Enter
Sleep 2s
