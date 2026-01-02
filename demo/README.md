# CDD Demo Recording

This directory contains demo recordings for the CDD (Context-Driven Development) tool.

## Available Demos

- `cdd-demo.cast` - Automated demo showing CDD installation and usage
- `demo-fast.sh` - Fast demo script (no delays)
- `demo-commands.sh` - Full demo script with delays for manual recording

## Viewing the Demo

### Play Demo (Works Anywhere)
```bash
just demo-play
```
This runs the demo script directly - works in any environment!

### Replay in Terminal (Requires TTY)
```bash
just demo-replay
```

Or directly with asciinema:
```bash
asciinema play demo/cdd-demo.cast
```

### Convert to Media Files

**GIF (animated, shareable)**
```bash
just demo-gif
```
Output: `demo/cdd-demo.gif`

**MP4 (video format)**
```bash
just demo-mp4
```
Output: `demo/cdd-demo.mp4`

### Upload to asciinema.org
```bash
just demo-upload
```

## Recording New Demos

### Automated Recording with VHS (Recommended)
```bash
just demo-record
```
Creates `demo/cdd-demo.gif` directly using the `demo.tape` script.

### Manual Interactive Recording
```bash
just demo-interactive
```
Records your terminal session - you control everything manually.

**Optional: Show keystrokes in recording with KeyCastr**

KeyCastr displays keystrokes as an overlay on your screen while you type.

```bash
# Install KeyCastr (if not already installed)
brew install --cask keycastr

# Start KeyCastr
open -a KeyCastr

# Configure KeyCastr:
# - Position: Bottom center
# - Font size: Medium
# - Display duration: 2 seconds

# Then record your demo
just demo-interactive
```

KeyCastr will show each keystroke at the bottom of your screen during recording.

## Demo Content

The demo showcases:
1. CDD help and version information
2. Installing CDD with OpenCode profile
3. Verifying the installation structure
4. Creating sample tasks
5. Overview of the task selector

## Requirements

- **VHS** (automated demos): `brew install vhs`
- **asciinema** (manual demos): `brew install asciinema`
- **agg** (convert .cast to GIF): `cargo install --git https://github.com/asciinema/agg`
- **ffmpeg** (video conversion): `brew install ffmpeg`

## Cleaning Up

```bash
just demo-clean
```
