@echo off
setlocal enabledelayedexpansion

echo ========================================================
echo        Building Persica Setup Installer
echo ========================================================
echo.

cd /d "%~dp0.."

echo [1/2] Compiling Persica Release binary...
cargo build --release
if errorlevel 1 (
    echo [ERROR] Cargo release build failed!
    pause
    exit /b 1
)

echo.
echo [2/2] Compiling Windows Setup Installer with Inno Setup...

set "ISCC_PATH="

where iscc >nul 2>nul
if %errorlevel% equ 0 (
    set "ISCC_PATH=iscc"
) else if exist "F:\Program Files\Inno Setup 7\ISCC.exe" (
    set "ISCC_PATH=F:\Program Files\Inno Setup 7\ISCC.exe"
) else if exist "C:\Program Files\Inno Setup 7\ISCC.exe" (
    set "ISCC_PATH=C:\Program Files\Inno Setup 7\ISCC.exe"
) else if exist "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" (
    set "ISCC_PATH=C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
)

if "%ISCC_PATH%"=="" (
    echo [ERROR] Inno Setup compiler (ISCC.exe) was not found!
    echo Please install Inno Setup or make sure ISCC.exe is accessible.
    pause
    exit /b 1
)

"%ISCC_PATH%" "%~dp0persica.iss"
if errorlevel 1 (
    echo [ERROR] Inno Setup compilation failed!
    pause
    exit /b 1
)

echo.
echo ========================================================
echo [SUCCESS] Installer generated in 'dist' directory!
echo ========================================================
echo.
pause
