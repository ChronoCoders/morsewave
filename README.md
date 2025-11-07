# MorseWave

**Retro Telegraph Reimagined** - A high-performance Morse code encoder/decoder built with Rust and WebAssembly.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![WebAssembly](https://img.shields.io/badge/wasm-enabled-green.svg)
![Platform](https://img.shields.io/badge/platform-web-lightgrey.svg)

## Overview

MorseWave combines vintage telegraph aesthetics with modern web technology to create an immersive Morse code learning and communication tool. Built with Rust for core logic and compiled to WebAssembly for browser performance, it features a retro-futuristic CRT interface with real-time audio and visual feedback.

## Features

### Core Functionality
- **Text to Morse Encoding** - Convert any text message to Morse code instantly
- **Morse to Text Decoding** - Translate Morse code back to readable text
- **Real-time Validation** - Automatic syntax checking for Morse code input
- **Character Support** - Full alphabet, numbers, and common punctuation

### Audio & Visual
- **Audio Transmission** - Play Morse code with authentic 800Hz sine wave tones
- **LED Indicator** - Visual feedback synchronized with audio playback
- **Signal Monitor** - Real-time waveform visualization on canvas
- **Adjustable Speed** - 5-40 WPM (Words Per Minute) control
- **Volume Control** - 0-100% audio level adjustment

### Interactive Features
- **Manual Telegraph** - Send Morse code with dot/dash buttons
- **Keyboard Shortcuts** - Fast input with J/K keys or dot/dash symbols
- **Auto-transmit Mode** - Automatic playback on encoding
- **Transmission Log** - History of all messages with timestamps
- **One-click Copy** - Copy encoded Morse to clipboard

### User Interface
- **Retro-Futuristic Design** - CRT screen effects with phosphor green glow
- **Scanline Animation** - Authentic vintage monitor simulation
- **Responsive Layout** - Works on desktop and mobile devices
- **Morse Reference Table** - Quick lookup for all supported characters
- **Persistent Settings** - LocalStorage for history and preferences

## Technology Stack

| Component | Technology |
|-----------|-----------|
| Core Logic | Rust 2021 Edition |
| Compilation | WebAssembly (WASM) |
| Audio | Web Audio API |
| Visualization | Canvas API |
| Storage | LocalStorage API |
| Build Tool | wasm-pack |
| Frontend | Vanilla JavaScript |

## Installation

### Prerequisites

- Rust 1.70 or higher
- Python 3.x (for local server)
- Modern web browser with WASM support

### Quick Start

1. Clone the repository:
```bash
git clone https://github.com/chronocoder/morsewave.git
cd morsewave
```

2. Build and run:
```cmd
build.bat
```

The build script will:
- Install wasm-pack if needed
- Compile Rust to WebAssembly
- Start a local server
- Open your browser automatically

### Manual Build

```bash
cargo install wasm-pack
wasm-pack build --target web --out-dir www/pkg
cd www
python -m http.server 8080
```

Open `http://localhost:8080` in your browser.

### Running After Build

For subsequent runs, use the start script:
```cmd
start.bat
```

## Usage

### Basic Encoding
1. Type your message in the TRANSMITTER panel
2. Click **ENCODE** to convert to Morse code
3. Click **▶ TRANSMIT** to play audio
4. Watch the LED and signal monitor for visual feedback

### Basic Decoding
1. Enter Morse code in the RECEIVER panel
   - Use `.` for dot
   - Use `-` for dash
   - Use `/` for word space
2. Click **DECODE** to translate
3. View the decoded message

### Manual Telegraph
- Click **DOT** button or press `.` or `J` key
- Click **DASH** button or press `-` or `K` key
- Click **SPACE** or press Space bar
- Watch real-time decoding as you type

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `.` or `J` | Dot |
| `-` or `K` | Dash |
| Space | Character space |

## Morse Code Reference

### Letters
```
A .-    B -...  C -.-.  D -..   E .     F ..-.
G --.   H ....  I ..    J .---  K -.-   L .-..
M --    N -.    O ---   P .--.  Q --.-  R .-.
S ...   T -     U ..-   V ...-  W .--   X -..-
Y -.--  Z --..
```

### Numbers
```
0 -----  1 .----  2 ..---  3 ...--  4 ....-
5 .....  6 -....  7 --...  8 ---..  9 ----.
```

### Timing
- Dot duration: 1 unit
- Dash duration: 3 units
- Gap between elements: 1 unit
- Gap between letters: 3 units
- Gap between words: 7 units

## Project Structure

```
morsewave/
├── src/
│   └── lib.rs              # Rust core logic
├── www/
│   ├── index.html          # Main interface
│   ├── favicon.svg         # Site icon
│   ├── css/
│   │   └── style.css       # Retro-futuristic styling
│   ├── js/
│   │   └── app.js          # Application logic
│   └── pkg/                # Generated WASM files
├── Cargo.toml              # Rust dependencies
├── build.bat               # Build automation
├── start.bat               # Server startup
└── README.md               # Documentation
```

## Configuration

### Speed Control
Adjust WPM (Words Per Minute) from 5 to 40:
- 5 WPM: Beginner friendly
- 20 WPM: Standard practice speed
- 40 WPM: Advanced operator speed

Formula: `dot_duration_ms = 1200 / WPM`

### Audio Settings
- Frequency: 800 Hz sine wave
- Volume: 0-100% adjustable
- Type: Web Audio API OscillatorNode

## Browser Compatibility

| Browser | Support |
|---------|---------|
| Chrome 57+ | ✓ Full |
| Firefox 52+ | ✓ Full |
| Safari 11+ | ✓ Full |
| Edge 16+ | ✓ Full |
| Opera 44+ | ✓ Full |

Requirements:
- WebAssembly support
- Web Audio API
- Canvas API
- LocalStorage

## Performance

- WASM bundle size: ~50KB (optimized)
- Initial load time: <100ms
- Encoding/Decoding: <1ms
- Memory usage: <10MB
- Audio latency: <50ms

## Development

### Build for Release
```bash
wasm-pack build --target web --release --out-dir www/pkg
```

### Debug Mode
```bash
wasm-pack build --target web --dev --out-dir www/pkg
```

### Run Tests
```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- International Morse Code standard (ITU-R M.1677-1)
- Rust WebAssembly working group
- Web Audio API specification

## Contact

Project Link: [https://github.com/chronocoder/morsewave](https://github.com/chronocoder/morsewave)

---

**MorseWave** - Bridging the past and future of communication

Built with ❤️ using Rust and WebAssembly