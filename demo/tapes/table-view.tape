# VHS tape: recon table view with 6 agents
# Run: vhs demo/table-view.tape
# Prerequisite: ./demo/demo.sh --setup  (in another terminal)

Output demo/out/table-view.gif
Set Shell zsh
Set FontSize 14
Set Width 1400
Set Height 500
Set Padding 10
Set Theme "Catppuccin Mocha"

Type "recon"
Enter
Sleep 3s

# Navigate down through sessions
Sleep 500ms
Type "j"
Sleep 600ms
Type "j"
Sleep 600ms
Type "j"
Sleep 600ms
Type "j"
Sleep 600ms
Type "j"
Sleep 600ms

# Navigate back up
Type "k"
Sleep 400ms
Type "k"
Sleep 400ms
Type "k"
Sleep 400ms
Type "k"
Sleep 400ms
Type "k"
Sleep 1s

Type "q"
Sleep 500ms
