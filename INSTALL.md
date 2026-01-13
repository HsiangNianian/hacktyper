# Installation Guide

This guide provides detailed information about installing hacktyper on various platforms.

## Quick Install

### Linux & macOS

```bash
curl -fsSL https://raw.githubusercontent.com/HsiangNianian/hacktyper/master/install.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/HsiangNianian/hacktyper/master/install.ps1 | iex
```

## Supported Platforms

The one-click install scripts support the following platforms:

| OS | Architecture | Package Formats |
| --- | --- | --- |
| Linux | x86_64 (amd64) | .deb, .rpm, .tar.gz |
| Linux | aarch64 (arm64) | .deb, .rpm, .tar.gz |
| Linux | armv7 (armhf/armv7hl) | .deb, .rpm, .tar.gz |
| macOS | x86_64 (Intel) | .tar.gz |
| macOS | aarch64 (Apple Silicon) | .tar.gz |
| Windows | x86_64 | .zip |
| Windows | aarch64 (WoA) | .zip |

## Installation Process

### Linux

The install script automatically:

1. Detects your Linux distribution
2. Chooses the appropriate package format:
   - **Debian/Ubuntu**: `.deb` package (uses `apt` or `dpkg`)
   - **RHEL/Fedora/CentOS**: `.rpm` package (uses `dnf`, `yum`, or `rpm`)
   - **Other distros**: `.tar.gz` archive (extracts to `/usr/local/bin`)
3. Downloads the latest release
4. Installs using the appropriate package manager or extracts to system path
5. Verifies the installation

**Installation directories:**
- Package installations (.deb, .rpm): Installed to `/usr/bin/hacktyper` by the package manager
- Archive installations (.tar.gz): Installed to `/usr/local/bin/hacktyper`

### macOS

The install script:

1. Detects your Mac architecture (Intel or Apple Silicon)
2. Downloads the appropriate .tar.gz archive
3. Extracts to `/usr/local/bin`
4. Verifies the installation

**Note:** You may need to allow the binary in System Preferences → Security & Privacy if you see a security warning.

### Windows

The install script (PowerShell):

1. Detects your Windows architecture
2. Downloads the appropriate .zip archive
3. Extracts to `%LOCALAPPDATA%\Programs\hacktyper`
4. Adds the installation directory to your PATH
5. Verifies the installation

**Note:** You may need to restart your terminal for PATH changes to take effect.

## Manual Installation

If you prefer to install manually or the automatic script doesn't work for your system:

### Download Pre-built Binaries

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

### Install the Downloaded Package

#### Debian/Ubuntu (.deb)

```bash
sudo dpkg -i hacktyper-linux-*.deb
sudo apt-get install -f  # Fix any dependency issues
```

#### RHEL/Fedora/CentOS (.rpm)

```bash
# Using dnf (Fedora)
sudo dnf install hacktyper-linux-*.rpm

# Using yum (older systems)
sudo yum install hacktyper-linux-*.rpm

# Using rpm directly
sudo rpm -ivh hacktyper-linux-*.rpm
```

#### Linux/macOS (.tar.gz)

```bash
tar -xzf hacktyper-*.tar.gz
sudo mv hacktyper /usr/local/bin/
sudo chmod +x /usr/local/bin/hacktyper
```

#### Windows (.zip)

1. Extract the zip file
2. Move `hacktyper.exe` to a directory in your PATH, or
3. Add the directory containing `hacktyper.exe` to your PATH

## Build from Source

If you have Rust installed, you can build from source:

### From crates.io

```bash
cargo install hacktyper
```

### From GitHub

```bash
git clone https://github.com/HsiangNianian/hacktyper
cd hacktyper
cargo build --release
```

The binary will be in `target/release/hacktyper`.

## Troubleshooting

### Linux: Permission Denied

If you get a permission denied error, you may need to use `sudo`:

```bash
curl -fsSL https://raw.githubusercontent.com/HsiangNianian/hacktyper/master/install.sh | sudo bash
```

### macOS: Security Warning

If macOS blocks the binary, go to System Preferences → Security & Privacy and click "Allow Anyway".

### Windows: Execution Policy

If the PowerShell script is blocked, you may need to temporarily allow script execution:

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Then run the install script again.

### Command Not Found After Installation

If `hacktyper` is not found after installation:

1. **Linux/macOS**: Ensure `/usr/local/bin` is in your PATH
2. **Windows**: Restart your terminal or run `refreshenv` (if using Chocolatey)

You can verify your PATH with:
- Linux/macOS: `echo $PATH`
- Windows: `$env:PATH` (PowerShell) or `echo %PATH%` (CMD)

### Install to Custom Directory

You can customize the installation directory:

**Linux/macOS:**
```bash
INSTALL_DIR="$HOME/.local/bin" bash install.sh
```

**Windows:**
Edit the `$INSTALL_DIR` variable in `install.ps1` before running.

## Uninstallation

### Package Installations

**Debian/Ubuntu:**
```bash
sudo apt remove hacktyper
```

**RHEL/Fedora:**
```bash
sudo dnf remove hacktyper  # or: sudo yum remove hacktyper
```

### Manual Installations

Simply remove the binary:

**Linux/macOS:**
```bash
sudo rm /usr/local/bin/hacktyper
```

**Windows:**
Remove the installation directory and update your PATH if needed.

## Support

If you encounter any issues with installation, please:

1. Check the [Troubleshooting](#troubleshooting) section above
2. Open an issue on [GitHub](https://github.com/HsiangNianian/hacktyper/issues) with:
   - Your operating system and version
   - Your architecture (x86_64, aarch64, etc.)
   - The complete error message
   - The output of the install script
