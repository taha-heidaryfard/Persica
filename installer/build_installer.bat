@echo off
setlocal enabledelayedexpansion

echo ========================================================
echo        Building Persica Setup Installer
echo ========================================================
echo.

cd /d "%~dp0.."

echo [1/2] Checking Cargo and compiling Persica Release binary...

set "CARGO_CMD=cargo"
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    if exist "%USERPROFILE%\.cargo\bin\cargo.exe" (
        set "CARGO_CMD=%USERPROFILE%\.cargo\bin\cargo.exe"
        set "PATH=%USERPROFILE%\.cargo\bin;!PATH!"
    ) else if exist "target\release\persica.exe" (
        echo [WARNING] Cargo not found in PATH, but existing target\release\persica.exe found.
        echo Using existing release binary...
        goto :inno_step
    ) else (
        echo [ERROR] Cargo command not found!
        echo Please install Rust or add your .cargo\bin directory to PATH.
        pause
        exit /b 1
    )
)

"%CARGO_CMD%" build --release
if errorlevel 1 (
    echo [ERROR] Cargo release build failed!
    pause
    exit /b 1
)

:inno_step
echo.
echo [2/2] Compiling Windows Setup Installer with Inno Setup...

set "ISCC_PATH="
where iscc >nul 2>nul
if %errorlevel% equ 0 set "ISCC_PATH=iscc" & goto :iscc_found
if exist "F:\Program Files\Inno Setup 7\ISCC.exe" set "ISCC_PATH=F:\Program Files\Inno Setup 7\ISCC.exe" & goto :iscc_found
if exist "C:\Program Files\Inno Setup 7\ISCC.exe" set "ISCC_PATH=C:\Program Files\Inno Setup 7\ISCC.exe" & goto :iscc_found
if exist "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" set "ISCC_PATH=C:\Program Files (x86)\Inno Setup 6\ISCC.exe" & goto :iscc_found

echo [ERROR] Inno Setup compiler ISCC.exe was not found!
echo Please install Inno Setup or make sure ISCC.exe is accessible.
pause
exit /b 1

:iscc_found
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
