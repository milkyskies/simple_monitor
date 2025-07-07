# Simple System Monitor

A lightweight cross-platform system monitoring tool with a web API, built in Rust.

![CI](https://github.com/yourusername/simple_monitor/workflows/CI/badge.svg)

## Features

- **CPU Monitoring**: Per-core usage, averages, and CPU information
- **Memory Monitoring**: Used/total memory with percentages
- **GPU Monitoring**: NVIDIA GPU usage, memory, temperature (if available)
- **Web API**: RESTful endpoints for easy integration
- **Cross-Platform**: Runs on Linux, Windows, and macOS
- **Configurable**: Environment variable configuration
- **Lightweight**: Single binary, minimal resource usage

## Quick Start

### Download Pre-built Binaries

1. Go to the [Releases](https://github.com/yourusername/simple_monitor/releases) page
2. Download the appropriate binary for your platform:

   - **Linux**: `simple_monitor-linux-x86_64.tar.gz`
   - **Windows**: `simple_monitor-windows-x86_64.exe.zip`
   - **macOS Intel**: `simple_monitor-macos-x86_64.tar.gz`
   - **macOS Apple Silicon**: `simple_monitor-macos-arm64.tar.gz`

3. Extract and run:

   ```bash
   # Linux/macOS
   tar -xzf simple_monitor-*.tar.gz
   ./simple_monitor-*

   # Windows
   # Extract the zip file and run simple_monitor.exe
   # Or use the included start-windows.bat
   ```

### Build from Source

```bash
git clone https://github.com/yourusername/simple_monitor.git
cd simple_monitor
cargo build --release
./target/release/simple_monitor
```

## Configuration

### Easy Configuration (Windows)

Run the interactive configuration script:

```cmd
configure-windows.bat
```

### Manual Configuration

Create a `.env` file or set environment variables:

```bash
# Copy the example configuration
cp env.example .env

# Edit .env
HOST=127.0.0.1    # Use 0.0.0.0 to allow external connections
PORT=3000         # Change the port
```

### Detailed Configuration Guide

See [DEPLOYMENT.md](DEPLOYMENT.md) for comprehensive instructions on setting environment variables across different platforms and deployment scenarios.

## API Endpoints

Once running, access these endpoints:

- **Health Check**: `GET /`

  ```
  http://localhost:3000/
  ```

- **System Stats**: `GET /stats`
  ```
  http://localhost:3000/stats
  ```

### Example Response

```json
{
  "cpu_usage": {
    "cores_total": 8,
    "cores_usage": [15.2, 23.1, 18.7, 25.9, 19.3, 22.1, 17.8, 20.4],
    "average_usage_percentage": 20.31,
    "brand": "Apple M1"
  },
  "memory_usage": {
    "used_bytes": 12857032704,
    "total_bytes": 17179869184,
    "used_percentage": 74.84
  },
  "gpu_usage": {
    "name": "NVIDIA RTX 4080",
    "memory_used_bytes": 2147483648,
    "memory_total_bytes": 17179869184,
    "memory_used_percentage": 12.5,
    "utilization_percentage": 45,
    "temperature_celsius": 67
  },
  "timestamp": 1751854049
}
```

## Deployment & Services

- **Environment Variables**: See [DEPLOYMENT.md](DEPLOYMENT.md) for all configuration methods
- **Windows Service**: See [build-windows.md](build-windows.md) for service installation
- **Linux/macOS Services**: Systemd and launchd examples in [DEPLOYMENT.md](DEPLOYMENT.md)

## Building for Different Platforms

### Cross-compilation from macOS/Linux

```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Install mingw-w64 (macOS)
brew install mingw-w64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## Dependencies

- **sysinfo**: System information
- **nvml-wrapper**: NVIDIA GPU monitoring
- **axum**: Web framework
- **tokio**: Async runtime
- **serde**: JSON serialization
- **dotenv**: Environment variable loading

## GPU Support

- **NVIDIA**: Full support via NVIDIA Management Library (NVML)
- **AMD/Intel**: Not currently supported (contributions welcome!)

GPU monitoring will gracefully degrade if no NVIDIA GPU is detected.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Release Process

1. Update version in `Cargo.toml`
2. Create and push a git tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
3. GitHub Actions will automatically build and create a release
