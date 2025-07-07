@echo off
REM ================================================
REM Simple Monitor - Windows Startup Script
REM ================================================
REM 
REM CONFIGURATION: Edit the values below or use a .env file
REM 
REM HOST Options:
REM   127.0.0.1 = Local access only (default)
REM   0.0.0.0   = Allow access from other devices on network
REM 
REM PORT Options:
REM   Any number 1024-65535 (default: 3000)
REM ================================================

REM Check if .env file exists first
if exist ".env" (
    echo Found .env file - using settings from there
    echo.
) else (
    echo No .env file found - using settings below
    echo To use .env file: copy env.example to .env and edit it
    echo.
    
    REM Set default values if not already defined
    if not defined HOST set HOST=127.0.0.1
    if not defined PORT set PORT=3000
)

echo ================================================
echo          Simple System Monitor
echo ================================================
echo Host: %HOST%
echo Port: %PORT%
echo.
echo Starting server...
echo.
echo Access your monitor at:
echo   Local:    http://localhost:%PORT%
echo   Network:  http://%HOST%:%PORT%
echo.
echo Endpoints:
echo   Health:   http://%HOST%:%PORT%/
echo   Stats:    http://%HOST%:%PORT%/stats
echo.
echo Press Ctrl+C to stop the server
echo ================================================
echo.

REM Start the application
simple_monitor.exe

REM If the program exits, pause so user can see any error messages
echo.
echo ================================================
echo Program ended. Press any key to close...
pause >nul 