Output assets/demo.gif

Set Shell "zsh"
Set Theme "Catppuccin Mocha"
Set Width 1200
Set Height 700
Set FontSize 15
Set Padding 20
Set WindowBar Colorful
Set BorderRadius 8
Set TypingSpeed 50ms
Set Framerate 24

# Build and launch hidden so the GIF starts with the app
Hide
Type "cargo build --example basic --release 2>/dev/null"
Enter
Sleep 8s
Type "./target/release/examples/basic"
Enter
Sleep 2s
Show

# Let the layout breathe, monitor bars and logs are animating
Sleep 3s

# Navigate between all panes
Type "h"
Sleep 400ms
Type "j"
Sleep 400ms
Type "k"
Sleep 400ms
Type "l"
Sleep 500ms

# Switch to input mode and type in the editor
Escape
Sleep 300ms
Type@80ms "Hello from Hypertile!"
Sleep 800ms
Escape
Sleep 500ms

# Move monitor to the right (swap with editor)
Type "h"
Sleep 300ms
Type "L"
Sleep 700ms

# Move it back
Type "H"
Sleep 700ms

# Focus logs and resize it smaller
Type "j"
Sleep 300ms
Type "["
Sleep 150ms
Type "["
Sleep 150ms
Type "["
Sleep 500ms

# Split logs and add help via palette
Type "s"
Sleep 500ms
Enter
Sleep 400ms
Type@80ms "help"
Sleep 300ms
Enter
Sleep 1.2s

# Move help pane up
Type "K"
Sleep 700ms

# Resize help bigger
Type "]"
Sleep 150ms
Type "]"
Sleep 150ms
Type "]"
Sleep 600ms

# Navigate to editor
Type "l"
Sleep 300ms
Type "k"
Sleep 400ms

# Split editor and add another monitor
Type "v"
Sleep 500ms
Enter
Sleep 400ms
Type@80ms "mon"
Sleep 300ms
Enter
Sleep 1.5s

# Let the 5-pane layout breathe
Sleep 1.5s

# Close the new monitor
Type "d"
Sleep 500ms

# Close help
Type "j"
Sleep 300ms
Type "d"
Sleep 500ms

# Grow logs back
Type "]"
Sleep 150ms
Type "]"
Sleep 150ms
Type "]"
Sleep 800ms

# Let the final clean layout breathe
Sleep 1.5s

# Quit (hidden so GIF ends on the app, not the terminal)
Hide
Type "q"
Sleep 500ms
