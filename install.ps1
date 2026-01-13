# One-click install script for hacktyper on Windows
# Supports Windows x86_64 and aarch64 (WoA)

#Requires -Version 5.1

# Configuration
$ErrorActionPreference = "Stop"
$ProgressPreference = 'SilentlyContinue'

$REPO = "HsiangNianian/hacktyper"
$INSTALL_DIR = "$env:LOCALAPPDATA\Programs\hacktyper"
$BIN_DIR = "$env:LOCALAPPDATA\Microsoft\WindowsApps"

# Helper functions
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
    exit 1
}

# Detect architecture
function Get-Architecture {
    $arch = $env:PROCESSOR_ARCHITECTURE.ToLower()
    
    switch ($arch) {
        "amd64" { return "x86_64" }
        "x86_64" { return "x86_64" }
        "arm64" { return "aarch64" }
        default {
            Write-Error-Custom "Unsupported architecture: $arch"
        }
    }
}

# Get latest release version
function Get-LatestVersion {
    Write-Info "Fetching latest release version..."
    
    try {
        # Set up headers for GitHub API
        $headers = @{
            "Accept" = "application/vnd.github+json"
            "User-Agent" = "hacktyper-installer"
        }
        
        # Use GitHub token if available (for CI environments)
        if ($env:GITHUB_TOKEN) {
            $headers["Authorization"] = "Bearer $env:GITHUB_TOKEN"
        }
        
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest" -Headers $headers
        $version = $response.tag_name
        
        if ([string]::IsNullOrEmpty($version)) {
            Write-Error-Custom "Failed to fetch latest version"
        }
        
        Write-Info "Latest version: $version"
        return $version
    }
    catch {
        Write-Error-Custom "Failed to fetch release information: $_"
    }
}

# Download the release
function Download-Release {
    param(
        [string]$Version,
        [string]$Arch,
        [string]$TempDir
    )
    
    $fileName = "hacktyper-windows-$Arch.zip"
    $downloadUrl = "https://github.com/$REPO/releases/download/$Version/$fileName"
    $outputPath = Join-Path $TempDir $fileName
    
    Write-Info "Downloading $fileName..."
    Write-Info "URL: $downloadUrl"
    
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $outputPath -UseBasicParsing
        Write-Info "Download complete"
        return $outputPath
    }
    catch {
        Write-Error-Custom "Failed to download release: $_"
    }
}

# Extract and install
function Install-Hacktyper {
    param(
        [string]$ZipPath,
        [string]$InstallDir,
        [string]$BinDir
    )
    
    Write-Info "Installing hacktyper..."
    
    # Create installation directory if it doesn't exist
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    
    # Extract the zip file
    try {
        Expand-Archive -Path $ZipPath -DestinationPath $InstallDir -Force
        Write-Info "Extracted to $InstallDir"
    }
    catch {
        Write-Error-Custom "Failed to extract archive: $_"
    }
    
    # Find the binary
    $binary = Get-ChildItem -Path $InstallDir -Filter "hacktyper.exe" -Recurse | Select-Object -First 1
    
    if ($null -eq $binary) {
        Write-Error-Custom "Could not find hacktyper.exe in the archive"
    }
    
    # Move binary to root of install directory if it's in a subdirectory
    $targetBinary = Join-Path $InstallDir "hacktyper.exe"
    if ($binary.FullName -ne $targetBinary) {
        Move-Item -Path $binary.FullName -Destination $targetBinary -Force
    }
    
    Write-Info "Installed hacktyper to $targetBinary"
    
    # Add to PATH if not already there
    Add-ToPath -InstallDir $InstallDir
}

# Add installation directory to PATH
function Add-ToPath {
    param([string]$InstallDir)
    
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    
    if ($userPath -notlike "*$InstallDir*") {
        Write-Info "Adding $InstallDir to PATH..."
        
        $newPath = if ([string]::IsNullOrEmpty($userPath)) {
            $InstallDir
        } else {
            "$userPath;$InstallDir"
        }
        
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        $env:PATH = "$env:PATH;$InstallDir"
        
        Write-Info "Added to PATH. You may need to restart your terminal for changes to take effect."
    } else {
        Write-Info "Installation directory already in PATH"
    }
}

# Verify installation
function Test-Installation {
    param([string]$InstallDir)
    
    Write-Info "Verifying installation..."
    
    $hacktypePath = Join-Path $InstallDir "hacktyper.exe"
    
    if (Test-Path $hacktypePath) {
        try {
            $version = & $hacktypePath --version 2>&1
            Write-Info "✓ hacktyper installed successfully!"
            Write-Info "Version: $version"
            Write-Host ""
            Write-Host "You can now run: hacktyper" -ForegroundColor Cyan
            
            # Check if it's in PATH
            $inPath = $null -ne (Get-Command hacktyper -ErrorAction SilentlyContinue)
            
            if (-not $inPath) {
                Write-Warn "If 'hacktyper' command is not found, restart your terminal or run:"
                Write-Host "  $hacktypePath" -ForegroundColor Yellow
            }
        }
        catch {
            Write-Warn "hacktyper was installed but version check failed: $_"
            Write-Host "Installation path: $hacktypePath" -ForegroundColor Cyan
        }
    }
    else {
        Write-Error-Custom "Installation verification failed - binary not found at $hacktypePath"
    }
}

# Main execution
function Main {
    Write-Host "╔════════════════════════════════════════╗"
    Write-Host "║  Hacktyper One-Click Installer        ║"
    Write-Host "║           Windows Edition              ║"
    Write-Host "╚════════════════════════════════════════╝"
    Write-Host ""
    
    # Detect architecture
    $arch = Get-Architecture
    Write-Info "Detected architecture: $arch"
    
    # Create temporary directory
    $tempDir = Join-Path $env:TEMP "hacktyper-install-$(Get-Random)"
    New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
    
    try {
        # Get latest version
        $version = Get-LatestVersion
        
        # Download release
        $zipPath = Download-Release -Version $version -Arch $arch -TempDir $tempDir
        
        # Install
        Install-Hacktyper -ZipPath $zipPath -InstallDir $INSTALL_DIR -BinDir $BIN_DIR
        
        # Verify
        Test-Installation -InstallDir $INSTALL_DIR
        
        Write-Host ""
        Write-Info "Installation complete! 🎉"
    }
    finally {
        # Cleanup
        if (Test-Path $tempDir) {
            Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

# Run main function
Main
