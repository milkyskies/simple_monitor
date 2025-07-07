# Deployment Guide

## Setting Environment Variables

### Windows

#### Method 1: .env File (Recommended - Easiest)

1. Copy `env.example` to `.env` in the same folder as your `.exe`
2. Edit `.env`:
   ```
   HOST=0.0.0.0
   PORT=8080
   ```
3. Run `simple_monitor.exe` - it will automatically load the `.env` file

#### Method 2: Command Line (Temporary)

```cmd
# Set for current session only
set HOST=0.0.0.0
set PORT=8080
simple_monitor.exe
```

#### Method 3: PowerShell (Temporary)

```powershell
# Set for current session only
$env:HOST="0.0.0.0"
$env:PORT="8080"
.\simple_monitor.exe
```

#### Method 4: Batch File (Easy for Users)

Create `start.bat`:

```batch
@echo off
REM Configure your settings here
set HOST=0.0.0.0
set PORT=8080

echo Starting Simple Monitor on %HOST%:%PORT%
simple_monitor.exe
pause
```

#### Method 5: User Environment Variables (Permanent for User)

1. Press `Win + R`, type `sysdm.cpl`, press Enter
2. Click "Environment Variables"
3. Under "User variables", click "New"
4. Add:
   - Variable name: `HOST`
   - Variable value: `0.0.0.0`
5. Click "New" again
6. Add:
   - Variable name: `PORT`
   - Variable value: `8080`
7. Click OK, restart Command Prompt
8. Run `simple_monitor.exe`

#### Method 6: System Environment Variables (Permanent for All Users)

1. Press `Win + R`, type `sysdm.cpl`, press Enter
2. Click "Environment Variables"
3. Under "System variables", click "New"
4. Add `HOST` and `PORT` as above
5. **Requires Administrator privileges**

#### Method 7: Registry (Advanced)

```cmd
# Run as Administrator
reg add "HKCU\Environment" /v HOST /t REG_SZ /d "0.0.0.0"
reg add "HKCU\Environment" /v PORT /t REG_SZ /d "8080"
```

#### Method 8: Windows Service with NSSM

```cmd
# When installing as service, set environment variables
nssm install SimpleMonitor "C:\path\to\simple_monitor.exe"
nssm set SimpleMonitor AppEnvironmentExtra "HOST=0.0.0.0" "PORT=8080"
nssm start SimpleMonitor
```

### Linux

#### Method 1: .env File (Recommended)

```bash
cp env.example .env
# Edit .env with your preferred editor
nano .env
./simple_monitor
```

#### Method 2: Command Line (Temporary)

```bash
HOST=0.0.0.0 PORT=8080 ./simple_monitor
```

#### Method 3: Export (Session)

```bash
export HOST=0.0.0.0
export PORT=8080
./simple_monitor
```

#### Method 4: User Profile (Permanent)

Add to `~/.bashrc` or `~/.profile`:

```bash
export HOST=0.0.0.0
export PORT=8080
```

#### Method 5: System-wide (All Users)

Add to `/etc/environment`:

```bash
HOST=0.0.0.0
PORT=8080
```

#### Method 6: Systemd Service

Create `/etc/systemd/system/simple-monitor.service`:

```ini
[Unit]
Description=Simple Monitor
After=network.target

[Service]
Type=simple
User=nobody
Environment=HOST=0.0.0.0
Environment=PORT=8080
ExecStart=/usr/local/bin/simple_monitor
Restart=always

[Install]
WantedBy=multi-user.target
```

### macOS

#### Method 1: .env File (Recommended)

```bash
cp env.example .env
# Edit .env
nano .env
./simple_monitor
```

#### Method 2: Command Line

```bash
HOST=0.0.0.0 PORT=8080 ./simple_monitor
```

#### Method 3: User Profile

Add to `~/.zshrc` or `~/.bash_profile`:

```bash
export HOST=0.0.0.0
export PORT=8080
```

#### Method 4: launchd Service

Create `~/Library/LaunchAgents/com.yourname.simple-monitor.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.yourname.simple-monitor</string>
    <key>ProgramArguments</key>
    <array>
        <string>/path/to/simple_monitor</string>
    </array>
    <key>EnvironmentVariables</key>
    <dict>
        <key>HOST</key>
        <string>0.0.0.0</string>
        <key>PORT</key>
        <string>8080</string>
    </dict>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

## Docker Deployment

### Dockerfile

```dockerfile
FROM scratch
COPY simple_monitor /
EXPOSE 3000
ENV HOST=0.0.0.0
ENV PORT=3000
CMD ["/simple_monitor"]
```

### Docker Run

```bash
docker run -p 8080:3000 -e HOST=0.0.0.0 -e PORT=3000 simple_monitor
```

### Docker Compose

```yaml
version: "3.8"
services:
  monitor:
    image: simple_monitor
    ports:
      - "8080:3000"
    environment:
      - HOST=0.0.0.0
      - PORT=3000
    restart: unless-stopped
```

## Common Deployment Scenarios

### 1. Personal Use (Local Only)

```
HOST=127.0.0.1
PORT=3000
```

Access: http://localhost:3000

### 2. Home Network Access

```
HOST=0.0.0.0
PORT=3000
```

Access: http://192.168.1.100:3000 (your computer's IP)

### 3. Public Server

```
HOST=0.0.0.0
PORT=8080
```

Access: http://your-server-ip:8080
**Note**: Consider firewall rules and security!

### 4. Behind Reverse Proxy

```
HOST=127.0.0.1
PORT=3000
```

Let nginx/Apache handle external access

## Verification

After setting environment variables, verify they're loaded:

### Windows

```cmd
echo %HOST%
echo %PORT%
```

### Linux/macOS

```bash
echo $HOST
echo $PORT
```

The application will also show the bind address when it starts:

```
System Monitor server running on http://0.0.0.0:8080
```
