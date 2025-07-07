# Building for Windows and Autostart Setup

## Building for Windows

### Option 1: Cross-compile from macOS/Linux

1. Install the Windows target:

```bash
rustup target add x86_64-pc-windows-gnu
```

2. Install mingw-w64 (on macOS):

```bash
brew install mingw-w64
```

3. Build for Windows:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

The executable will be at: `target/x86_64-pc-windows-gnu/release/simple_monitor.exe`

### Option 2: Build natively on Windows

1. Install Rust on Windows from https://rustup.rs/
2. Clone the repository
3. Build:

```cmd
cargo build --release
```

## Environment Variables on Windows

### Method 1: Create a .env file

Copy `env.example` to `.env` and modify as needed:

```
HOST=0.0.0.0
PORT=8080
```

### Method 2: Set system environment variables

```cmd
setx HOST "0.0.0.0"
setx PORT "8080"
```

### Method 3: Set for current session only

```cmd
set HOST=0.0.0.0
set PORT=8080
simple_monitor.exe
```

## Windows Autostart Options

### Option 1: Windows Startup Folder (User-level)

1. Press `Win + R`, type `shell:startup`, press Enter
2. Copy `simple_monitor.exe` to this folder
3. Create a batch file `start_monitor.bat`:

```batch
@echo off
cd /d "C:\path\to\your\app"
simple_monitor.exe
```

### Option 2: Windows Service (System-level)

1. Install NSSM (Non-Sucking Service Manager): https://nssm.cc/
2. Install as service:

```cmd
nssm install SimpleMonitor "C:\path\to\simple_monitor.exe"
nssm set SimpleMonitor AppDirectory "C:\path\to\your\app"
nssm set SimpleMonitor Start SERVICE_AUTO_START
nssm start SimpleMonitor
```

### Option 3: Task Scheduler (System-level)

1. Open Task Scheduler
2. Create Basic Task
3. Set trigger to "When the computer starts"
4. Set action to start `simple_monitor.exe`
5. Set "Start in" directory to your app folder

### Option 4: Registry Run Key (User-level)

1. Open Registry Editor (regedit)
2. Navigate to: `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run`
3. Create new String Value: `SimpleMonitor`
4. Set value to: `C:\path\to\simple_monitor.exe`

## Running as Windows Service (Recommended)

For production use, running as a Windows Service is recommended. Here's a complete setup:

1. Download NSSM and extract to a folder
2. Open Command Prompt as Administrator
3. Install the service:

```cmd
nssm install SimpleMonitor "C:\path\to\simple_monitor.exe"
nssm set SimpleMonitor AppDirectory "C:\path\to\your\app"
nssm set SimpleMonitor AppEnvironmentExtra "HOST=0.0.0.0" "PORT=3000"
nssm set SimpleMonitor Start SERVICE_AUTO_START
nssm set SimpleMonitor DisplayName "Simple System Monitor"
nssm set SimpleMonitor Description "System monitoring web service"
```

4. Start the service:

```cmd
nssm start SimpleMonitor
```

5. Check service status:

```cmd
nssm status SimpleMonitor
```

To remove the service:

```cmd
nssm stop SimpleMonitor
nssm remove SimpleMonitor confirm
```
