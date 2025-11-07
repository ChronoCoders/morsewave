@echo off
color 0A
cls

echo.
echo ====================================================
echo           MORSEWAVE AUTOMATIC SETUP
echo        Retro Telegraph Reimagined
echo ====================================================
echo.

echo [STEP 1/6] Checking Rust installation...
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust is NOT installed
    echo.
    echo Please install Rust first:
    echo    https://rustup.rs/
    echo.
    echo Or use winget: winget install Rustlang.Rust.GNU
    echo.
    pause
    exit /b 1
) else (
    echo [OK] Rust is installed
)
echo.

echo [STEP 2/6] Installing wasm-pack...
cargo install wasm-pack
if %errorlevel% neq 0 (
    echo [ERROR] wasm-pack installation failed
    pause
    exit /b 1
) else (
    echo [OK] wasm-pack ready
)
echo.

echo [STEP 3/6] Building Rust to WebAssembly...
wasm-pack build --target web --out-dir www/pkg
if %errorlevel% neq 0 (
    echo [ERROR] Build failed
    pause
    exit /b 1
) else (
    echo [OK] WASM build successful
)
echo.

echo [STEP 4/6] Checking for Python...
python --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Python not found
    echo.
    echo Please install Python:
    echo    https://www.python.org/downloads/
    echo.
    pause
    exit /b 1
) else (
    echo [OK] Python is available
)
echo.

echo [STEP 5/6] Starting local server...
echo.
echo ====================================================
echo              SERVER IS STARTING
echo         http://localhost:8080
echo.
echo   Press Ctrl+C to stop the server
echo ====================================================
echo.

timeout /t 2 >nul

echo [STEP 6/6] Opening browser...
start http://localhost:8080
echo [OK] Browser opened
echo.

cd www
python -m http.server 8080
