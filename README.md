# About

WTBGC (**W**indows **Terminal** **B**ack**g**round **C**ycler) is a tool designed to change the background image of your [Windows Terminal](https://github.com/microsoft/terminal) periodically.
The project compiles to an executable that changes the background image everytime it's called. It's up to you to decide when to execute the program (see the [recommended setup](#recommended-setup) below).

TODO Image here

# Installation and use

- Download the latest zip file from the [releases](TODO) section.
- Extract the `wtgbc` folder and move it to your desired directory.
- [Optional] Modify [`config.toml`](#optional-configuration) if necessary.
- Put your background pictures into the `backgrounds/` directory.

# Recommended setup

There are two main ways to use this program.
One of them is to set up a scheduler to execute the program periodically or whenever your PC boots.
The other option (explained here) is to execute the program everytime the Windows Terminal opens up.

- Copy the full path of the executable into your clipboard (example: `C:\tools\wtgbc\wtgbc.exe`)
- Open Windows Terminal
- Go to Settings
- Create a new profile
- Click on the Command Line option
- Set it to `powershell -noexit <path>`, where `<path>` is the directory you have in your clipboard
    - Surround the path in escaped quotes (`\"`) if it contains any whitespace and, prefix it with `&` (example: `powershell -noexit & \"C:\Program Files\wtgbc\wtgbc.exe\"`)
    - If using `cmd.exe`, set the command as `cmd /k <path>`
- Click on Save
- Go to Startup
- Select your newly created profile as the default

# [Optional] Configuration

Edit `config.toml` if necessary:

- `backgrounds`: Path to backgrounds directory
    - Can be absolute or relative to the executable's directory
    - Default: `backgrounds/` (relative)
- `backups`: Path to the Windows Terminal `config.json` backups directory
    - Can be absolute or relative to the executable's directory
    - Default: `backups/` (relative)
- `wt_profile`: Path to the Windows Terminal `config.json`
    - Relative to `$HOME`
    - Default: `AppData/Local/Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json
- `extensions`: Valid image file extensions
    - Default: `[gif, png, jpg, jpeg]`
- `mode`: Determines the cycling mode (`random` or `cycle`)
    - WARNING: Currently has no effect
    - Default: `random`

# TODO

- Improve error handling
- Divide code into functions
- Add "cycle" mode
- Prevent image from appearing twice in a row
- Set timestamp on backup files

