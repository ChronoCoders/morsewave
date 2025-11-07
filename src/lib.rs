use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorseMessage {
    pub text: String,
    pub morse: String,
    pub timestamp: f64,
}

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

    pub fn encode(&self, text: &str) -> String {
        text.to_uppercase()
            .chars()
            .filter_map(|ch| self.encode_map.get(&ch))
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn decode(&self, morse: &str) -> String {
        morse
            .split(' ')
            .filter_map(|code| self.decode_map.get(code))
            .collect()
    }
}

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
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        MorseWave {
            codec: MorseCodec::new(),
        }
    }

    pub fn text_to_morse(&self, text: &str) -> String {
        self.codec.encode(text)
    }

    pub fn morse_to_text(&self, morse: &str) -> String {
        self.codec.decode(morse)
    }

    pub fn validate_morse(&self, morse: &str) -> bool {
        morse
            .split_whitespace()
            .all(|code| code.chars().all(|c| c == '.' || c == '-' || c == '/'))
    }
}

#[wasm_bindgen]
pub struct AudioPlayer {
    context: AudioContext,
    dot_duration: f64,
}

#[wasm_bindgen]
impl AudioPlayer {
    #[wasm_bindgen(constructor)]
    pub fn new(wpm: f64) -> Result<AudioPlayer, JsValue> {
        let context = AudioContext::new()?;
        let dot_duration = 1200.0 / wpm;

        Ok(AudioPlayer {
            context,
            dot_duration,
        })
    }

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

    pub fn set_wpm(&mut self, wpm: f64) {
        self.dot_duration = 1200.0 / wpm;
    }
}
