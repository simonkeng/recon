# VHS tape: short README hero GIF
# ~15 seconds: table → view → zoom → quit

Output readme.gif
Set Shell bash
Set FontSize 14
Set Width 1400
Set Height 700
Set Padding 10
Set Theme "Catppuccin Mocha"

Type "recon"
Sleep 1s
Enter
Sleep 2.5s

# Quick navigation
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "k"
Sleep 1s

# Switch to view mode
Type "v"
Sleep 3s

# Zoom into room 1 (api-server with 2 agents)
Type "1"
Sleep 2.5s

# Select agent
Type "l"
Sleep 1s

# Back out
Escape
Sleep 1.5s

# Zoom into room 2 (api-server › services/auth — Input)
Type "2"
Sleep 2s

Escape
Sleep 1s

Type "q"
Sleep 300ms
