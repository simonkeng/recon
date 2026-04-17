# VHS tape: recon demo — table view, visual view, zoom, navigation
# Runs inside Docker with fake sessions set up by demo.sh

Output tmux-demo.gif
Set Shell bash
Set FontSize 14
Set Width 1400
Set Height 700
Set Padding 10
Set Theme "Catppuccin Mocha"

# --- Table view ---
Type "recon"
Enter
Sleep 3s

# Navigate sessions
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "k"
Sleep 400ms
Type "k"
Sleep 1.5s

# --- Switch to visual view ---
Type "v"
Sleep 4s

# --- Zoom into room 1 (api-server with 2 agents) ---
Type "1"
Sleep 3s

# --- Select agents within room ---
Type "l"
Sleep 700ms
Type "h"
Sleep 1.5s

# --- Back to overview ---
Escape
Sleep 2s

# --- Zoom into room 2 ---
Type "2"
Sleep 3s

# --- Back to overview ---
Escape
Sleep 2s

# --- Back to table ---
Type "v"
Sleep 2s

# --- Quit ---
Type "q"
Sleep 500ms
