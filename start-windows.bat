@echo off
REM Simple Monitor Startup Script for Windows
REM You can modify the environment variables below or use a .env file

REM Set default values (modify as needed)
if not defined HOST set HOST=127.0.0.1
if not defined PORT set PORT=3000

echo Starting Simple Monitor...
echo Host: %HOST%
echo Port: %PORT%
echo.
echo Access the monitor at: http://%HOST%:%PORT%
echo Health check: http://%HOST%:%PORT%/
echo System stats: http://%HOST%:%PORT%/stats
echo.
echo Press Ctrl+C to stop the server
echo.

REM Start the application
simple_monitor.exe

pause 