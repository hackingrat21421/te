# te (手)

> Your helping hand for command-line interfaces

`te` (Japanese: 手, "hand") is an interactive TUI wrapper that makes editing complex CLI commands easier by providing a form-based interface for modifying command arguments.

## The Problem

Long command-line commands are hard to edit and reuse:

```bash
# Want to change just one parameter in this long command?
kubectl get pods -l app=asset -o custom-columns='POD:.metadata.name,RS:.metadata.ownerReferences[0].name' -w

# Have to manually edit the entire command string
# Easy to make mistakes with quotes, commas, etc.
```

## The Solution

Simply prefix your existing command with `te`:

```bash
te kubectl get pods -l app=asset -o custom-columns='POD:.metadata.name,RS:.metadata.ownerReferences[0].name' -w
```

`te` will:
- 📋 Parse your existing command and extract all arguments and their values
- ✨ Present an interactive TUI form for editing values
- 💾 Display the modified command (without executing it)
- ⚡ Let you copy and run the command when ready

## Features

### 🎨 Interactive TUI
Beautiful terminal interface built with [ratatui](https://github.com/ratatui-org/ratatui) that shows:
- All command arguments with their current values
- Editable form fields for each argument
- Real-time command preview as you edit

### 🔧 Universal Wrapper
Works with any CLI command. `te` simply parses your command string - no special support needed from the tool.

### 🚀 Edit, Don't Execute
`te` focuses on helping you build the right command:
- **Shows the final command** instead of executing it
- **Copy-paste friendly** output
- **Safe to experiment** - no accidental command execution

## Installation

```bash
# With cargo
cargo install te-cli

# From source
git clone https://github.com/yusukeshib/te
cd te
cargo build --release
```

## Usage

### Basic Usage

Simply prefix your existing command with `te`:

```bash
# Edit a kubectl command
te kubectl get pods -l app=myapp -o json

# Edit a docker command
te docker run -d -p 8080:80 --name myapp -e ENV=prod nginx

# Edit an ffmpeg command
te ffmpeg -i input.mp4 -c:v libx264 -crf 23 output.mp4

# Even works with commands from history
te $(history | grep kubectl | tail -1 | cut -d' ' -f4-)
```

### In the TUI

- `↑/↓`: Navigate between arguments
- `Enter`: Edit the selected argument's value
- `Esc`: Cancel editing / Exit
- `Ctrl+X`: Confirm and display the final command

## How It Works

1. **Parse Command**: `te` parses your command line to extract the base command, subcommands, and all arguments with their values
2. **Present TUI**: Shows an interactive form with current values pre-filled
3. **Edit**: You modify the values you want to change
4. **Display**: Shows the final command string (does not execute it)
5. **Copy & Run**: You copy and run the command manually

## Comparison

| Tool | Scope | Features |
|------|-------|----------|
| AWS CLI `--cli-auto-prompt` | AWS only | Interactive prompts, resource suggestions |
| `kube-prompt` | kubectl only | Auto-complete |
| `trogon` | Python Click/Typer apps | Auto-generated TUI |
| **`te`** | **Any CLI tool** | **Interactive TUI + History + Presets** |

## Why "te" (手)?

In Japanese, 手 (te) means "hand" - representing:
- 🤝 A helping hand for complex commands
- ✋ Easy to type (just 2 characters)
- 🎌 Honoring the Unix philosophy with a Japanese touch

## Roadmap

### Current Phase (v0.1)
- [ ] Basic TUI interface
- [ ] Parse existing command arguments and values
- [ ] Pre-fill form with existing values
- [ ] Display-only mode (no execution)

### Future Features

- [ ] **Per-command configuration**
  - Custom labels and descriptions for specific arguments
  - Custom input types (dropdown, checkbox, file picker)
  - Validation rules for argument values
  - Example values
  - Provider argument options from a specified command in the config.toml

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) for details

## Credits

Built with:
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing

---

**Star ⭐ this repo if you find it useful!**
