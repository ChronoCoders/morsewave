@echo off
color 0A
cls

echo.
echo ====================================================
echo           MORSEWAVE SERVER STARTING
echo ====================================================
echo.

echo Checking for Python...
python --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Python not found
    echo.
    echo Please install Python:
    echo    https://www.python.org/downloads/
    echo.
    pause
    exit /b 1
)

echo [OK] Python is available
echo.
echo ====================================================
echo         Server running on port 8080
echo         http://localhost:8080
echo.
echo         Press Ctrl+C to stop
echo ====================================================
echo.

timeout /t 2 >nul

start http://localhost:8080

cd www
python -m http.server 8080