# Demo recording recipes

# Record automated demo using VHS (outputs GIF directly)
# Usage: just demo-record
demo-record:
    #!/usr/bin/env bash
    set -euo pipefail
    
    echo "📹 Recording automated demo with VHS..."
    
    # Check if VHS is installed
    if ! command -v vhs &> /dev/null; then
        echo "⚠️  VHS not found. Installing via Homebrew..."
        brew install vhs
    fi
    
    # Check if demo.tape exists
    if [ ! -f demo.tape ]; then
        echo "❌ Error: demo.tape not found"
        exit 1
    fi
    
    # Create demo directory
    mkdir -p demo
    
    # Record the demo
    vhs demo.tape
    
    echo "✅ Demo recorded to demo/cdd-demo.gif"
    echo "📊 File size: $(du -h demo/cdd-demo.gif | cut -f1)"
    echo ""
    echo "Next steps:"
    echo "  • View: open demo/cdd-demo.gif"
    echo "  • Convert to MP4: just demo-mp4"

# Record interactive manual demo session with asciinema
# Usage: just demo-interactive
demo-interactive:
    #!/usr/bin/env bash
    set -euo pipefail
    
    echo "🎬 Starting interactive demo recording..."
    echo "📝 You'll control the terminal manually"
    echo "⏹️  Press Ctrl+D when done recording"
    echo ""
    
    # Check if asciinema is installed
    if ! command -v asciinema &> /dev/null; then
        echo "❌ Error: asciinema is not installed"
        echo ""
        echo "Install it with:"
        echo "  macOS:   brew install asciinema"
        echo "  Linux:   apt install asciinema / dnf install asciinema"
        exit 1
    fi
    
    # Create demo directory
    mkdir -p demo
    
    # Start recording
    asciinema rec demo/cdd-demo.cast \
        --title "CDD - Context-Driven Development Tool" \
        --overwrite
    
    echo ""
    echo "✅ Interactive demo saved to: demo/cdd-demo.cast"
    echo ""
    echo "Next steps:"
    echo "  • Replay: just demo-replay"
    echo "  • Upload: just demo-upload"
    echo "  • Convert to GIF: just demo-gif"

# Replay the recorded demo
demo-replay:
    #!/usr/bin/env bash
    set -euo pipefail
    
    if [ ! -f demo/cdd-demo.cast ]; then
        echo "❌ Error: No recording found at demo/cdd-demo.cast"
        echo "Run 'just demo-record' first to create a recording"
        exit 1
    fi
    
    echo "▶️  Replaying demo..."
    asciinema play demo/cdd-demo.cast

# Play the demo in any context (works in OpenCode/non-TTY environments)
# Usage: just demo-play
demo-play:
    #!/usr/bin/env bash
    set -euo pipefail
    
    if [ ! -f demo/cdd-demo.cast ]; then
        echo "❌ Error: No recording found at demo/cdd-demo.cast"
        echo "Run 'just demo-interactive' first to create a recording"
        exit 1
    fi
    
    echo "▶️  Playing demo (running script directly)..."
    echo ""
    bash demo/demo-fast.sh

# Upload demo to asciinema.org (requires asciinema account)
# Usage: just demo-upload
demo-upload:
    #!/usr/bin/env bash
    set -euo pipefail
    
    if [ ! -f demo/cdd-demo.cast ]; then
        echo "❌ Error: No recording found at demo/cdd-demo.cast"
        echo "Run 'just demo-interactive' first to create a recording"
        exit 1
    fi
    
    echo "☁️  Uploading demo to asciinema.org..."
    echo ""
    
    if asciinema upload demo/cdd-demo.cast; then
        echo ""
        echo "✅ Upload successful!"
    else
        echo ""
        echo "⚠️  Upload failed. This could be due to:"
        echo "  - Temporary server issues with asciinema.org"
        echo "  - Network connectivity problems"
        echo "  - Account authentication issues"
        echo ""
        echo "Alternatives:"
        echo "  1. Try again later: just demo-upload"
        echo "  2. Convert to GIF: just demo-gif"
        echo "  3. Share the .cast file directly from demo/cdd-demo.cast"
        echo "  4. Use demo-play to run the demo locally: just demo-play"
        exit 1
    fi

# Convert demo to GIF (requires agg)
# Usage: just demo-gif
demo-gif:
    #!/usr/bin/env bash
    set -euo pipefail
    
    if [ ! -f demo/cdd-demo.cast ]; then
        echo "❌ Error: No recording found at demo/cdd-demo.cast"
        echo "Run 'just demo-interactive' first to create a recording"
        exit 1
    fi
    
    if ! command -v agg &> /dev/null; then
        echo "❌ Error: agg is not installed"
        echo ""
        echo "Install it with:"
        echo "  cargo install --git https://github.com/asciinema/agg"
        exit 1
    fi
    
    echo "🎨 Converting demo to GIF..."
    agg demo/cdd-demo.cast demo/cdd-demo.gif
    echo "✅ GIF saved to: demo/cdd-demo.gif"
    echo ""
    echo "File info:"
    ls -lh demo/cdd-demo.gif | awk '{print "  Size: " $5}'
    echo "  Location: $(pwd)/demo/cdd-demo.gif"

# Convert demo to MP4 (requires ffmpeg)
# Usage: just demo-mp4
demo-mp4:
    #!/usr/bin/env bash
    set -euo pipefail
    
    if [ ! -f demo/cdd-demo.gif ]; then
        echo "⚠️  GIF not found. Creating it first..."
        just demo-gif
        echo ""
    fi
    
    if ! command -v ffmpeg &> /dev/null; then
        echo "❌ Error: ffmpeg is not installed"
        echo ""
        echo "Install it with:"
        echo "  macOS:   brew install ffmpeg"
        echo "  Linux:   apt install ffmpeg / dnf install ffmpeg"
        exit 1
    fi
    
    echo "🎬 Converting GIF to MP4..."
    ffmpeg -i demo/cdd-demo.gif -movflags faststart -pix_fmt yuv420p \
        -vf "scale=trunc(iw/2)*2:trunc(ih/2)*2" \
        demo/cdd-demo.mp4 -y -loglevel error
    
    echo "✅ MP4 saved to: demo/cdd-demo.mp4"
    echo ""
    echo "File info:"
    ls -lh demo/cdd-demo.mp4 | awk '{print "  Size: " $5}'
    echo "  Location: $(pwd)/demo/cdd-demo.mp4"

# Clean up demo files
demo-clean:
    #!/usr/bin/env bash
    echo "🧹 Cleaning demo files..."
    rm -rf demo/
    echo "✅ Demo directory removed"

# Show demo info
demo-info:
    #!/usr/bin/env bash
    echo "📹 CDD Demo Recording"
    echo ""
    echo "Available recipes:"
    echo "  just demo-record       - Record automated demo with VHS → GIF"
    echo "  just demo-interactive  - Record manual demo with asciinema"
    echo "  just demo-play         - Play demo (works in any environment)"
    echo "  just demo-replay       - Replay asciinema recording (requires TTY)"
    echo "  just demo-upload       - Upload to asciinema.org"
    echo "  just demo-gif          - Convert asciinema to GIF (requires agg)"
    echo "  just demo-mp4          - Convert GIF to MP4 (requires ffmpeg)"
    echo "  just demo-clean        - Remove all demo files"
    echo ""
    echo "Requirements:"
    echo "  - VHS (automated): brew install vhs"
    echo "  - asciinema (manual): brew install asciinema"
    echo "  - agg (GIF from .cast): cargo install --git https://github.com/asciinema/agg"
    echo "  - ffmpeg (MP4): brew install ffmpeg"
    echo ""
    if [ -f demo/cdd-demo.cast ]; then
        echo "📁 Current recording: demo/cdd-demo.cast"
        echo "   Size: $(du -h demo/cdd-demo.cast | cut -f1)"
        echo ""
        echo "View options:"
        echo "  • Local: just demo-play"
        echo "  • Terminal replay: asciinema play demo/cdd-demo.cast"
        echo "  • Share: Upload to asciinema.org or convert to GIF"
    else
        echo "📁 No recording found"
        echo "   Run: just demo-interactive"
    fi
