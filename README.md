# te (手)

> Your helping hand for command-line interfaces

`te` (Japanese: 手, "hand") is an interactive TUI wrapper that makes complex CLI commands easier to use by prompting you for required arguments and showing you all available options.

## The Problem

Command-line tools are powerful but often hard to remember:

```bash
# Which arguments were required again?
aws ec2 run-instances --help  # scroll through walls of text...

# Building complex commands from memory
docker run -d -p 8080:80 --name myapp -e ENV=prod -v /data:/data nginx
```

## The Solution

With `te`, just type the command and it will guide you:

```bash
te aws ec2 run-instances
```

`te` will:
- 📋 Parse the command's help to identify required and optional arguments
- ✨ Present an interactive TUI for filling in values
- 🎯 Remember your frequently used options
- 💾 Save command history for quick reuse
- ⚡ Generate the final command with one keystroke

## Features

### 🎨 Interactive TUI
Beautiful terminal interface built with [ratatui](https://github.com/ratatui-org/ratatui) that shows:
- Required arguments (must fill)
- Optional arguments (choose what you need)
- Argument descriptions and types
- Real-time command preview

### 🧠 Smart Suggestions
- **Frequency-based sorting**: Most-used options appear first
- **Context-aware completions**: Suggests valid values based on your environment
- **Default values**: Mark commonly used values as defaults

### 📚 History & Presets
```bash
# Quick access to command history
te history

# Save frequently used command patterns
te save-preset my-ec2-dev

# Reuse saved presets
te aws ec2 run-instances --preset my-ec2-dev
```

### 🔧 Universal Wrapper
Works with any CLI tool:
- `aws` - AWS CLI
- `kubectl` - Kubernetes
- `docker` - Docker
- `ffmpeg` - Video processing
- `git` - Version control
- Any command-line tool with `--help`

## Installation

```bash
# With cargo
cargo install te

# From source
git clone https://github.com/yourusername/te
cd te
cargo build --release
```

## Usage

### Basic Usage

```bash
# Wrap any command
te <command> [subcommands...]

# Examples
te aws s3 cp
te kubectl create deployment
te docker run
te ffmpeg -i
```

### Command History

```bash
# View and reuse previous commands
te history

# Search history
te history --search "ec2"
```

### Presets

```bash
# Save current configuration as preset
te save-preset <name>

# Use preset
te <command> --preset <name>

# List all presets
te list-presets
```

## Configuration

Configuration is stored in `~/.te/`:

```
~/.te/
├── config.toml           # Global settings
├── history.db            # Command execution history
├── presets/              # Saved command presets
│   ├── aws-dev.yaml
│   └── kubectl-prod.yaml
└── schemas/              # Custom command schemas (optional)
    └── custom-tool.yaml
```

### Example config.toml

```toml
[general]
# Enable frequency-based sorting
smart_sort = true

# Save command history
save_history = true

# Maximum history entries
max_history = 1000

[ui]
# Color scheme: "default", "nord", "dracula"
theme = "default"

# Show preview pane
show_preview = true
```

## How It Works

1. **Parse**: `te` runs `<command> --help` and parses the output to extract arguments
2. **Present**: Shows an interactive TUI with all options
3. **Build**: Constructs the final command based on your input
4. **Execute**: Runs the command or copies it to clipboard

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

- [x] Basic TUI interface
- [x] Help parsing
- [x] Command history
- [ ] Preset management
- [ ] Smart suggestions
- [ ] Context-aware completions
- [ ] Shell integration (bash, zsh, fish)
- [ ] Custom schema support
- [ ] Team preset sharing

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
