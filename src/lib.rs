//! # MorseWave
//!
//! A high-performance Morse code encoder/decoder library built with Rust and WebAssembly.
//!
//! MorseWave provides blazing-fast Morse code encoding and decoding capabilities,
//! along with real-time audio playback using the Web Audio API.
//!
//! ## Features
//!
//! - **Fast Encoding/Decoding**: Convert text to Morse code and back in <1ms
//! - **Audio Playback**: Real-time Morse code audio synthesis
//! - **WebAssembly Support**: Compile to WASM for browser usage
//! - **Full Character Set**: Supports letters, numbers, and punctuation
//!
//! ## Example
//!
//! ```rust
//! use morsewave::MorseWave;
//!
//! let morse = MorseWave::new();
//! let encoded = morse.text_to_morse("HELLO");
//! assert_eq!(encoded, ".... . .-.. .-.. ---");
//!
//! let decoded = morse.morse_to_text(".... . .-.. .-.. ---");
//! assert_eq!(decoded, "HELLO");
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

/// Represents a Morse code message with its text, encoded form, and timestamp.
///
/// # Fields
///
/// * `text` - The original text message
/// * `morse` - The Morse code representation
/// * `timestamp` - Unix timestamp in milliseconds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorseMessage {
    pub text: String,
    pub morse: String,
    pub timestamp: f64,
}

/// Core Morse code encoder and decoder.
///
/// Provides bidirectional conversion between text and Morse code using
/// International Morse Code standard (ITU-R M.1677-1).
///
/// # Example
///
/// ```rust
/// use morsewave::MorseCodec;
///
/// let codec = MorseCodec::new();
/// let morse = codec.encode("SOS");
/// assert_eq!(morse, "... --- ...");
/// ```
pub struct MorseCodec {
    encode_map: HashMap<char, &'static str>,
    decode_map: HashMap<&'static str, char>,
}

impl Default for MorseCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl MorseCodec {
    /// Creates a new MorseCodec with standard International Morse Code mappings.
    ///
    /// Initializes lookup tables for encoding and decoding.
    ///
    /// # Example
    ///
    /// ```rust
    /// use morsewave::MorseCodec;
    /// let codec = MorseCodec::new();
    /// ```
    pub fn new() -> Self {
        let pairs = vec![
            ('A', ".-"),
            ('B', "-..."),
            ('C', "-.-."),
            ('D', "-.."),
            ('E', "."),
            ('F', "..-."),
            ('G', "--."),
            ('H', "...."),
            ('I', ".."),
            ('J', ".---"),
            ('K', "-.-"),
            ('L', ".-.."),
            ('M', "--"),
            ('N', "-."),
            ('O', "---"),
            ('P', ".--."),
            ('Q', "--.-"),
            ('R', ".-."),
            ('S', "..."),
            ('T', "-"),
            ('U', "..-"),
            ('V', "...-"),
            ('W', ".--"),
            ('X', "-..-"),
            ('Y', "-.--"),
            ('Z', "--.."),
            ('0', "-----"),
            ('1', ".----"),
            ('2', "..---"),
            ('3', "...--"),
            ('4', "....-"),
            ('5', "....."),
            ('6', "-...."),
            ('7', "--..."),
            ('8', "---.."),
            ('9', "----."),
            ('.', ".-.-.-"),
            (',', "--..--"),
            ('?', "..--.."),
            ('!', "-.-.--"),
            ('/', "-..-."),
            ('(', "-.--."),
            (')', "-.--.-"),
            ('&', ".-..."),
            (':', "---..."),
            (';', "-.-.-."),
            ('=', "-...-"),
            ('+', ".-.-."),
            ('-', "-....-"),
            ('_', "..--.-"),
            ('"', ".-..-."),
            ('$', "...-..-"),
            ('@', ".--.-."),
            (' ', "/"),
        ];

        let mut encode_map = HashMap::new();
        let mut decode_map = HashMap::new();

        for (ch, morse) in pairs.iter() {
            encode_map.insert(*ch, *morse);
            decode_map.insert(*morse, *ch);
        }

        MorseCodec {
            encode_map,
            decode_map,
        }
    }

    /// Encodes text into Morse code.
    ///
    /// Converts uppercase letters, numbers, and punctuation to Morse code.
    /// Unknown characters are silently ignored.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to encode
    ///
    /// # Returns
    ///
    /// Morse code string with spaces between letters and '/' for word spaces
    ///
    /// # Example
    ///
    /// ```rust
    /// use morsewave::MorseCodec;
    /// let codec = MorseCodec::new();
    /// assert_eq!(codec.encode("HELLO"), ".... . .-.. .-.. ---");
    /// ```
    pub fn encode(&self, text: &str) -> String {
        text.to_uppercase()
            .chars()
            .filter_map(|ch| self.encode_map.get(&ch))
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Decodes Morse code back into text.
    ///
    /// Converts Morse code sequences into their corresponding characters.
    /// Invalid Morse sequences are silently ignored.
    ///
    /// # Arguments
    ///
    /// * `morse` - The Morse code to decode (space-separated)
    ///
    /// # Returns
    ///
    /// Decoded text string
    ///
    /// # Example
    ///
    /// ```rust
    /// use morsewave::MorseCodec;
    /// let codec = MorseCodec::new();
    /// assert_eq!(codec.decode("... --- ..."), "SOS");
    /// ```
    pub fn decode(&self, morse: &str) -> String {
        morse
            .split(' ')
            .filter_map(|code| self.decode_map.get(code))
            .collect()
    }
}

/// WebAssembly-compatible Morse code interface.
///
/// Provides WASM bindings for encoding, decoding, and validating Morse code
/// in browser environments.
///
/// # Example
///
/// ```javascript
/// import init, { MorseWave } from './pkg/morsewave.js';
///
/// await init();
/// const morse = new MorseWave();
/// console.log(morse.text_to_morse("HELLO"));
/// ```
#[wasm_bindgen]
pub struct MorseWave {
    codec: MorseCodec,
}

impl Default for MorseWave {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl MorseWave {
    /// Creates a new MorseWave instance.
    ///
    /// Initializes panic hook for better error messages in browser console.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        MorseWave {
            codec: MorseCodec::new(),
        }
    }

    /// Converts text to Morse code.
    ///
    /// # Arguments
    ///
    /// * `text` - Text to encode
    ///
    /// # Returns
    ///
    /// Morse code string
    pub fn text_to_morse(&self, text: &str) -> String {
        self.codec.encode(text)
    }

    /// Converts Morse code to text.
    ///
    /// # Arguments
    ///
    /// * `morse` - Morse code to decode
    ///
    /// # Returns
    ///
    /// Decoded text string
    pub fn morse_to_text(&self, morse: &str) -> String {
        self.codec.decode(morse)
    }

    /// Validates Morse code syntax.
    ///
    /// Checks if input contains only valid Morse characters (., -, /, space).
    ///
    /// # Arguments
    ///
    /// * `morse` - Morse code to validate
    ///
    /// # Returns
    ///
    /// true if valid, false otherwise
    pub fn validate_morse(&self, morse: &str) -> bool {
        morse
            .split_whitespace()
            .all(|code| code.chars().all(|c| c == '.' || c == '-' || c == '/'))
    }
}

/// Web Audio API-based Morse code audio player.
///
/// Generates authentic Morse code tones using sine wave oscillators.
/// Supports adjustable speed (WPM) and standard timing rules.
///
/// # Example
///
/// ```javascript
/// const player = new AudioPlayer(20); // 20 WPM
/// player.play_morse("... --- ...");
/// ```
#[wasm_bindgen]
pub struct AudioPlayer {
    context: AudioContext,
    dot_duration: f64,
}

#[wasm_bindgen]
impl AudioPlayer {
    /// Creates a new AudioPlayer with specified speed.
    ///
    /// # Arguments
    ///
    /// * `wpm` - Words per minute (5-40 recommended)
    ///
    /// # Returns
    ///
    /// Result containing AudioPlayer or JsValue error
    ///
    /// # Formula
    ///
    /// dot_duration_ms = 1200 / WPM
    #[wasm_bindgen(constructor)]
    pub fn new(wpm: f64) -> Result<AudioPlayer, JsValue> {
        let context = AudioContext::new()?;
        let dot_duration = 1200.0 / wpm;

        Ok(AudioPlayer {
            context,
            dot_duration,
        })
    }

    /// Plays Morse code audio.
    ///
    /// Synthesizes audio for dots, dashes, and spaces with proper timing.
    ///
    /// # Arguments
    ///
    /// * `morse` - Morse code string to play
    ///
    /// # Returns
    ///
    /// Result indicating success or error
    ///
    /// # Timing
    ///
    /// * Dot: 1 unit
    /// * Dash: 3 units
    /// * Gap between elements: 1 unit
    /// * Gap between letters: 3 units (space)
    /// * Gap between words: 7 units (/)
    pub fn play_morse(&self, morse: &str) -> Result<(), JsValue> {
        let mut time = self.context.current_time();

        for ch in morse.chars() {
            match ch {
                '.' => {
                    self.play_tone(time, self.dot_duration)?;
                    time += self.dot_duration / 1000.0;
                    time += self.dot_duration / 1000.0;
                }
                '-' => {
                    self.play_tone(time, self.dot_duration * 3.0)?;
                    time += (self.dot_duration * 3.0) / 1000.0;
                    time += self.dot_duration / 1000.0;
                }
                ' ' => {
                    time += (self.dot_duration * 3.0) / 1000.0;
                }
                '/' => {
                    time += (self.dot_duration * 7.0) / 1000.0;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Plays a single tone at specified time and duration.
    ///
    /// # Arguments
    ///
    /// * `start_time` - AudioContext time to start
    /// * `duration` - Duration in milliseconds
    ///
    /// # Audio Properties
    ///
    /// * Frequency: 800 Hz sine wave
    /// * Volume: 0.3 gain
    fn play_tone(&self, start_time: f64, duration: f64) -> Result<(), JsValue> {
        let oscillator = self.context.create_oscillator()?;
        let gain_node = self.context.create_gain()?;

        oscillator.set_type(web_sys::OscillatorType::Sine);

        let freq_param = oscillator.frequency();
        freq_param.set_value(800.0);

        let gain_param = gain_node.gain();
        gain_param.set_value(0.3);

        oscillator.connect_with_audio_node(&gain_node)?;
        gain_node.connect_with_audio_node(&self.context.destination())?;

        oscillator.start_with_when(start_time)?;
        oscillator.stop_with_when(start_time + duration / 1000.0)?;

        Ok(())
    }

    /// Updates playback speed.
    ///
    /// # Arguments
    ///
    /// * `wpm` - New words per minute speed
    pub fn set_wpm(&mut self, wpm: f64) {
        self.dot_duration = 1200.0 / wpm;
    }
}
