# Hacker Typer

A sophisticated CLI tool for pranking or acting like a Hollywood hacker.

## Features

- **Auto-Typing**: Mash the keyboard to type valid code (or any text) on the screen.
- **Mechanical Sound**: Procedural audio synthesis mimics mechanical keyboard switches.
- **Multi-Window Mode**: Spawn multiple windows to look busy.
- **Custom Scripts**: Load any text file (e.g., source code, movie scripts).

## Installation

### One-Click Install (Recommended)

**Linux & macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/HsiangNianian/hacktyper/master/install.sh | bash
```

Or download and run:

```bash
wget https://raw.githubusercontent.com/HsiangNianian/hacktyper/master/install.sh
chmod +x install.sh
./install.sh
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/HsiangNianian/hacktyper/master/install.ps1 | iex
```

Or download and run:

```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/HsiangNianian/hacktyper/master/install.ps1 -OutFile install.ps1
.\install.ps1
```

The install script automatically:
- Detects your OS and architecture
- Downloads the appropriate binary from the latest release
- Installs using the best method for your system (.deb, .rpm, or .tar.gz for Linux)
- Adds hacktyper to your PATH

**Supported Platforms:**
- Linux: x86_64, aarch64, armv7 (Debian/Ubuntu, RHEL/Fedora, and other distros)
- macOS: x86_64 (Intel), aarch64 (Apple Silicon)
- Windows: x86_64, aarch64 (WoA)

### Alternative Installation Methods

1. From crates.io

```bash
cargo install hacktyper
```

2. From source code

```bash
git clone https://github.com/HsiangNianian/hacktyper
cd hacktyper
cargo build --release
```

The binary will be in `target/release/hacktyper`.

## Usage

### Basic usage (Matrix Mode)

```bash
./hacktyper
```

### Type out a custom file (e.g., Harry Potter)

```bash
./hacktyper --file harry_potter.txt
```

### Hollywood Mode (Spawn 4 extra windows)

```bash
./hacktyper --multi-window --window-count 4
```

### Adjust Speed

```bash
./hacktyper --speed 5
```

## Options

- `-f, --file <FILE>`: Path to custom file.
- `-s, --speed <SPEED>`: Characters per keypress (default: 3).
- `--sound`: Boolean flag to enable/disable sound (default: true).
- `--multi-window`: Spawns multiple terminals.
- `--window-count <NUM>`: Number of terminals to spawn.
- `-m, --matrix`: Enable matrix effect.