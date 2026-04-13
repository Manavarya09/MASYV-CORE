# HELIOS - Futuristic AI Command System

<div align="center">

[![Version](https://img.shields.io/badge/version-0.8.0-blue)](https://github.com/Manavarya09/HELIOS/releases)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/Manavarya09/HELIOS/actions)
[![Security](https://img.shields.io/badge/security-audit-passed-green)](https://github.com/Manavarya09/HELIOS/security)
[![Last Commit](https://img.shields.io/badge/last%20commit-April%202026-orange)](https://github.com/Manavarya09/HELIOS/commits/main)
[![Stars](https://img.shields.io/badge/stars-0-yellow)](https://github.com/Manavarya09/HELIOS/stargazers)
[![Forks](https://img.shields.io/badge/forks-0-green)](https://github.com/Manavarya09/HELIOS/network/members)

A futuristic, AI-powered command system built with **Rust** and **egui**. HELIOS provides a professional GUI interface with real-time system metrics, JARVIS AI core, neon styling, and extensive command capabilities.

[🚀 Quick Start](#quick-start) • [📖 Documentation](#documentation) • [🤝 Contributing](#contributing) • [💬 Discussions](#discussions)

</div>

---

## 🌟 Features

### Core Features
- **File Operations** - ls, cd, pwd, read, write, mkdir, delete with full shell integration
- **Network Tools** - ping, curl, port scanning, network diagnostics
- **System Commands** - process management, system stats, resource monitoring
- **Command History** - searchable history with fuzzy search and timestamps
- **Alias System** - create and manage command aliases

### AI Integration
- **Multi-Provider Support** - Ollama, OpenAI, Anthropic, Claude, Gemini, DeepSeek
- **Holographic AI** - Advanced AI system with emotion states and voice synthesis
- **Chat History** - Persistent conversation context
- **Real-time Responses** - Streaming AI assistance

### UI/UX (Futuristic)
- **7 Themes** - Dark, Light, Cyberpunk, Ocean, Forest, Neon, Matrix
- **Neon Styling** - Glowing borders, colored progress bars, futuristic aesthetics
- **Real-Time Graphs** - CPU, Memory, Network, Power, GPU, Disk monitoring
- **JARVIS Core** - System status with encryption, shield, integrity monitoring
- **Keyboard Shortcuts** - Press `?` for help overlay
- **5-Panel Layout** - Navigation, Metrics, Status, Console, Actions

### Advanced Features
- **Plugin System** - Extensible architecture with hot-reload
- **Voice System** - TTS/STT support with configurable voice
- **Encryption** - XOR encryption for sensitive data
- **Security Scanner** - Vulnerability detection and reporting
- **Task Scheduler** - Scheduled task automation
- **Macro System** - Keyboard macro recording and playback
- **Persistent Config** - JSON-based settings with hot-reload

---

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+ 
- Cargo (included with Rust)

### Installation

```bash
# Clone the repository
git clone https://github.com/Manavarya09/HELIOS.git
cd HELIOS

# Build and run
cargo run --release
```

### Docker

```bash
# Build Docker image
docker build -t helios .

# Run container
docker run -p 8080:8080 helios
```

---

## 📖 Documentation

### Command Reference

#### General Commands
| Command | Description |
|---------|-------------|
| `help` | Display all available commands |
| `status` | Show system status |
| `clear` | Clear terminal output |
| `stats` | Show detailed system statistics |
| `time` | Show current system time |
| `history [cmd]` | Command history management |
| `alias [cmd]` | Alias management |
| `env [cmd]` | Environment variable management |
| `notes [cmd]` | Notes management |
| `todo [cmd]` | Todo list management |

#### File Operations
| Command | Description |
|---------|-------------|
| `ls [path]` | List directory contents |
| `cd <path>` | Change working directory |
| `pwd` | Print working directory |
| `read <file>` | Read file contents |
| `write <file> <content>` | Write to file |
| `mkdir <dir>` | Create directory |
| `delete <path>` | Delete file or directory |

#### Network Commands
| Command | Description |
|---------|-------------|
| `ping <host> [count]` | ICMP ping utility |
| `curl <url>` | HTTP request client |
| `scan <host> [start] [end]` | Port scanner |

#### AI Commands
| Command | Description |
|---------|-------------|
| `ai <prompt>` | Query AI assistant |
| `ai chat` | Toggle chat mode |
| `ai provider <name>` | Set AI provider |
| `ai model <name>` | Set AI model |
| `ai emotion <state>` | Set AI emotion |
| `ai voice on/off` | Toggle voice synthesis |

#### System Commands
| Command | Description |
|---------|-------------|
| `process list` | List running processes |
| `process kill <pid>` | Kill process |
| `system info` | Show system information |
| `task [cmd]` | Task scheduler |
| `macro [cmd]` | Macro management |

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `?` | Show help overlay |
| `T` | Cycle theme |
| `Esc` | Close overlay |
| `↑` / `↓` | Navigate command history |
| `Enter` | Execute command |

---

## 🏗️ Architecture

```
HELIOS/
├── src/
│   ├── ai/          # AI integration
│   ├── automation/   # Task scheduler & macros
│   ├── commands/    # Command system
│   ├── config/      # Configuration
│   ├── jarvis/      # JARVIS core
│   ├── output/      # Output formatting
│   ├── plugins/     # Plugin system
│   ├── security/    # Security modules
│   ├── system/      # System stats
│   ├── ui/          # UI components
│   └── lib.rs       # Main application
├── Cargo.toml
├── README.md
└── LICENSE
```

---

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) first.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing-feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 💬 Discussions

Join our [GitHub Discussions](https://github.com/Manavarya09/HELIOS/discussions) for:
- **Q&A** - Ask and answer questions
- **Ideas** - Suggest new features
- **Show and Tell** - Share your projects

---

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- [egui](https://github.com/emilk/egui) - Immediate mode GUI
- [Ollama](https://github.com/ollama/ollama) - Local AI
- [Rust](https://www.rust-lang.org/) - Systems programming

---

<div align="center">

**[⭐ Star us](https://github.com/Manavarya09/HELIOS/stargazers)** if you find this project useful!

*Built with ❤️ using Rust*

</div>