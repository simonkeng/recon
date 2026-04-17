# VHS tape: zoom into a room in view mode
# Run: vhs demo/zoom-room.tape
# Prerequisite: ./demo/demo.sh --setup  (in another terminal)

Output demo/out/zoom-room.gif
Set Shell zsh
Set FontSize 14
Set Width 1200
Set Height 700
Set Padding 10
Set Theme "Catppuccin Mocha"

Type "recon"
Enter
Sleep 2s

# Switch to view mode
Type "v"
Sleep 2s

# Zoom into room 1 (api-server with 2 agents)
Type "1"
Sleep 3s

# Navigate between agents
Type "l"
Sleep 800ms
Type "h"
Sleep 800ms

# Back to overview
Escape
Sleep 2s

# Zoom into room 2
Type "2"
Sleep 2s

Escape
Sleep 1s

Type "q"
Sleep 500ms
