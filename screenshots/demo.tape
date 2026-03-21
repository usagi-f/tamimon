Output screenshots/demo.gif

Set FontSize 16
Set FontFamily "NotoSansMonoCJKjp"
Set Width 720
Set Height 400
Set Padding 0
Set Theme "Catppuccin Mocha"
Set PlaybackSpeed 1

# Setup: put the demo save file in place
Type "mkdir -p /tmp/tamimon_home/.tamimon && cp /tmp/demo_save.json /tmp/tamimon_home/.tamimon/save.json"
Enter
Sleep 500ms

# Run tamimon with a custom HOME
Type "HOME=/tmp/tamimon_home /home/user/tamimon/target/release/tamimon"
Enter

# Wait for startup screen to render
Sleep 3s

# Press any key to go to main screen
Space

# Stay on main screen
Sleep 3s

# Quit
Type "q"
Sleep 500ms
