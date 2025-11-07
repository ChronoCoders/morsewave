# MorseWave Quick Start Guide

## Windows CMD Commands

### Step 1: Install Rust (if not installed)
```cmd
winget install Rustlang.Rust.GNU
```

### Step 2: Install wasm-pack
```cmd
cargo install wasm-pack
```

### Step 3: Build the project
```cmd
cd morsewave
wasm-pack build --target web --out-dir www/pkg
```

### Step 4: Run local server

#### Option A: Python (if installed)
```cmd
cd www
python -m http.server 8080
```

#### Option B: Node.js http-server
```cmd
npm install -g http-server
cd www
http-server -p 8080
```

#### Option C: PHP (if installed)
```cmd
cd www
php -S localhost:8080
```

### Step 5: Open browser
Navigate to: `http://localhost:8080`

## Quick Test

Once the application is running:

1. Type "HELLO WORLD" in the TRANSMITTER
2. Click "ENCODE"
3. Click "▶ TRANSMIT" to hear the Morse code
4. Watch the LED indicator flash!

## Keyboard Shortcuts

- Press `.` or `J` for DOT
- Press `-` or `K` for DASH
- Press `Space` for character separation

Enjoy MorseWave!
