# Hacker Typer (Rust Edition)

A sophisticated CLI tool for pranking or acting like a Hollywood hacker.

## Features

- **Auto-Typing**: Mash the keyboard to type valid code (or any text) on the screen.
- **Mechanical Sound**: Procedural audio synthesis mimics mechanical keyboard switches.
- **Multi-Window Mode**: Spawn multiple windows to look busy.
- **Custom Scripts**: Load any text file (e.g., source code, movie scripts).

## Installation

```bash
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
