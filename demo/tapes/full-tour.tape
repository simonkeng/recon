# VHS tape: full recon tour — table → view → zoom → back
# Run: vhs demo/full-tour.tape
# Prerequisite: ./demo/demo.sh --setup  (in another terminal)

Output demo/out/full-tour.gif
Set Shell zsh
Set FontSize 14
Set Width 1400
Set Height 700
Set Padding 10
Set Theme "Catppuccin Mocha"

Type "recon"
Enter
Sleep 3s

# Browse table view
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "j"
Sleep 400ms
Type "k"
Sleep 400ms
Type "k"
Sleep 1s

# Switch to view mode
Type "v"
Sleep 4s

# Zoom into room 1 (api-server with 2 agents)
Type "1"
Sleep 3s

# Select agents
Type "l"
Sleep 700ms

# Back to overview
Escape
Sleep 2s

# Back to table
Type "v"
Sleep 2s

Type "q"
Sleep 500ms
