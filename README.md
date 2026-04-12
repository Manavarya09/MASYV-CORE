# HELIOS - Futuristic Command System

<div align="center">

![Version](https://img.shields.io/badge/version-0.6.0-blue)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange)
![License](https://img.shields.io/badge/license-MIT-green)

HELIOS is a futuristic, AI-powered command system built with Rust and eframe/egui. It provides a professional GUI interface with real-time metrics, JARVIS AI core, neon styling, and extensive command capabilities.

</div>

## Features

### Core Features
- File Operations - ls, cd, pwd, read, write, mkdir, delete
- Network Tools - ping, curl, port scanning
- System Commands - process management, system stats
- Command History - searchable history with fuzzy search

### AI Integration
- Multi-Provider Support - Ollama, OpenAI, Anthropic
- Chat History - conversation context
- Real-time Responses - streaming AI assistance

### UI/UX (Futuristic)
- 7 Themes - Dark, Light, Cyberpunk, Ocean, Forest, Neon, Matrix
- Neon Styling - glowing borders, colored progress bars
- Real-Time Graphs - CPU, Memory, Network, Power, GPU, Disk
- JARVIS Core - system status with encryption, shield, integrity
- Keyboard Shortcuts - press ? for help
- 5-Panel Layout - Navigation, Metrics, Status, Console, Actions

### Advanced Features
- Plugin System - extensible architecture
- Persistent Config - JSON-based settings
- Alert System - critical/warning/info alerts
- Sparkline Graphs - ASCII visualization

## Quick Start

### Prerequisites
- Rust 1.75+
- Cargo

### Installation

```bash
# Clone the repository
git clone https://github.com/Manavarya09/MASYV-CORE.git
cd MASYV-CORE

# Build and run
cargo run --release
```

## Commands

### General Commands
| Command | Description |
|---------|-------------|
| help | Show all available commands |
| status | Show system status |
| clear | Clear terminal output |
| stats | Show detailed system stats |
| time | Show current time |
| history | Command history management |
| alias | Alias management |
| env | Environment variables |

### File Operations
| Command | Description |
|---------|-------------|
| ls [path] | List directory contents |
| cd <path> | Change directory |
| pwd | Print working directory |
| read <file> | Read file contents |
| write <file> <content> | Write to file |
| mkdir <dir> | Create directory |
| delete <path> | Delete file/directory |

### Network Tools
| Command | Description |
|---------|-------------|
| ping <host> [count] | Ping a host |
| curl <url> | Fetch URL content |
| scan <host> [start] [end] | Scan ports |

### System Commands
| Command | Description |
|---------|-------------|
| processes [count] | List running processes |
| kill <pid> | Kill a process |
| info <pid> | Get process info |

### AI Commands
| Command | Description |
|---------|-------------|
| ai <prompt> | Ask AI |
| ai provider <name> | Set AI provider |
| ai model <name> | Set AI model |
| ai history | Show chat history |
| ai clear | Clear chat history |

### Theme Commands
| Command | Description |
|---------|-------------|
| theme list | List all themes |
| theme set <name> | Set theme |
| theme next | Cycle to next theme |

### Config Commands
| Command | Description |
|---------|-------------|
| config list | List all settings |
| config get <key> | Get setting value |
| config set <key> <value> | Set setting |
| config save | Save config |
| config reset | Reset to defaults |

### Output Commands
| Command | Description |
|---------|-------------|
| format show | Show current format |
| format set <type> | Set output format |
| format demo | Demonstrate formats |

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Enter | Execute command |
| Arrow Up/Down | Navigate history |
| ? | Toggle shortcuts overlay |
| T | Cycle theme |
| Esc | Close overlay |

## Themes

- Dark (default) - Classic dark theme
- Light - Clean light theme
- Cyberpunk - Neon-inspired theme
- Ocean - Blue ocean theme
- Forest - Green forest theme

## Plugins

### Built-in Plugins
- FileManager - Advanced file operations
- NetworkTools - Network diagnostics
- ProcessManager - Process management

## AI Configuration

### Ollama (Default)
```bash
ai provider ollama
ai model llama2
```

### OpenAI
```bash
ai provider openai
ai apikey YOUR_API_KEY
ai model gpt-3.5-turbo
```

### Anthropic
```bash
ai provider anthropic
ai apikey YOUR_API_KEY
ai model claude-3-haiku
```

## Project Structure

```
masyv-core/
├── src/
│   ├── ai/           # AI integration
│   ├── commands/    # Command implementations
│   ├── config/      # Configuration system
│   ├── plugins/     # Plugin system
│   ├── system/       # System stats
│   ├── ui/          # UI/Theme system
│   └── lib.rs       # Main application
├── Cargo.toml       # Project config
└── README.md        # This file
```

## Development

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run
```

### Test
```bash
cargo test
```

## License

MIT License - see LICENSE file for details.

---

<div align="center">

HELIOS - Your AI-Powered Command System

</div>