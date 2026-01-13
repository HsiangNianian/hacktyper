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

#### Supported Platforms

| OS | Architecture | Package Formats |
| --- | --- | --- |
| Linux | x86_64 (amd64) | .deb, .rpm, .tar.gz |
| Linux | aarch64 (arm64) | .deb, .rpm, .tar.gz |
| Linux | armv7 (armhf/armv7hl) | .deb, .rpm, .tar.gz |
| macOS | x86_64 (Intel) | .tar.gz |
| macOS | aarch64 (Apple Silicon) | .tar.gz |
| Windows | x86_64 | .zip |
| Windows | aarch64 (WoA) | .zip |

### Manual Installation

If you prefer to install manually or the automatic script doesn't work for your system:

#### Download Pre-built Binaries

1. Go to the [latest release](https://github.com/HsiangNianian/hacktyper/releases/latest)
2. Download the appropriate file for your platform:
   - **Linux x86_64**: 
     - Debian/Ubuntu: `hacktyper-linux-amd64.deb`
     - RHEL/Fedora: `hacktyper-linux-x86_64.rpm`
     - Other: `hacktyper-linux-x86_64.tar.gz`
   - **Linux aarch64**:
     - Debian/Ubuntu: `hacktyper-linux-arm64.deb`
     - RHEL/Fedora: `hacktyper-linux-aarch64.rpm`
     - Other: `hacktyper-linux-aarch64.tar.gz`
   - **Linux armv7**:
     - Debian/Ubuntu: `hacktyper-linux-armhf.deb`
     - RHEL/Fedora: `hacktyper-linux-armv7hl.rpm`
     - Other: `hacktyper-linux-armv7hl.tar.gz`
   - **macOS Intel**: `hacktyper-macos-x86_64.tar.gz`
   - **macOS Apple Silicon**: `hacktyper-macos-aarch64.tar.gz`
   - **Windows x86_64**: `hacktyper-windows-x86_64.zip`
   - **Windows aarch64**: `hacktyper-windows-aarch64.zip`

#### Install the Downloaded Package

**Debian/Ubuntu (.deb):**

```bash
sudo dpkg -i hacktyper-linux-*.deb
sudo apt-get install -f  # Fix any dependency issues
```

**RHEL/Fedora/CentOS (.rpm):**

```bash
# Using dnf (Fedora)
sudo dnf install hacktyper-linux-*.rpm

# Using yum (older systems)
sudo yum install hacktyper-linux-*.rpm

# Using rpm directly
sudo rpm -ivh hacktyper-linux-*.rpm
```

**Linux/macOS (.tar.gz):**

```bash
tar -xzf hacktyper-*.tar.gz
sudo mv hacktyper /usr/local/bin/
sudo chmod +x /usr/local/bin/hacktyper
```

**Windows (.zip):**

1. Extract the zip file
2. Move `hacktyper.exe` to a directory in your PATH, or
3. Add the directory containing `hacktyper.exe` to your PATH

### Build from Source

If you have Rust installed, you can build from source:

**From crates.io:**

```bash
cargo install hacktyper
```

**From GitHub:**

```bash
git clone https://github.com/HsiangNianian/hacktyper
cd hacktyper
cargo build --release
```

The binary will be in `target/release/hacktyper`.

### Troubleshooting

**Linux: Permission Denied**

If you get a permission denied error, you may need to use `sudo`:

```bash
curl -fsSL https://raw.githubusercontent.com/HsiangNianian/hacktyper/master/install.sh | sudo bash
```

**macOS: Security Warning**

If macOS blocks the binary, go to System Preferences → Security & Privacy and click "Allow Anyway".

**Windows: Execution Policy**

If the PowerShell script is blocked, you may need to temporarily allow script execution:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Then run the install script again.

**Command Not Found After Installation**

If `hacktyper` is not found after installation:

1. **Linux/macOS**: Ensure `/usr/local/bin` is in your PATH
2. **Windows**: Restart your terminal or run `refreshenv` (if using Chocolatey)

You can verify your PATH with:
- Linux/macOS: `echo $PATH`
- Windows: `$env:PATH` (PowerShell) or `echo %PATH%` (CMD)

**Install to Custom Directory**

You can customize the installation directory:

```bash
# Linux/macOS
INSTALL_DIR="$HOME/.local/bin" bash install.sh
```

**Uninstallation**

Package installations:

```bash
# Debian/Ubuntu
sudo apt remove hacktyper

# RHEL/Fedora
sudo dnf remove hacktyper  # or: sudo yum remove hacktyper
```

Manual installations - simply remove the binary:

```bash
# Linux/macOS
sudo rm /usr/local/bin/hacktyper
```

For Windows, remove the installation directory and update your PATH if needed.

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