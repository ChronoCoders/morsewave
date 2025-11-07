import init, { MorseWave, AudioPlayer } from '../pkg/morsewave.js';

class MorseWaveApp {
    constructor() {
        this.morseWave = null;
        this.audioPlayer = null;
        this.wpm = 20;
        this.volume = 50;
        this.history = [];
        this.isPlaying = false;
        this.manualMorse = '';
        this.canvas = null;
        this.ctx = null;
        this.animationId = null;
    }

    async init() {
        await init();
        this.morseWave = new MorseWave();
        this.audioPlayer = new AudioPlayer(this.wpm);
        
        this.loadHistory();
        this.setupEventListeners();
        this.setupCanvas();
        this.generateMorseTable();
        this.startIdleAnimation();
    }

    setupEventListeners() {
        document.getElementById('encodeBtn').addEventListener('click', () => this.encodeText());
        document.getElementById('decodeBtn').addEventListener('click', () => this.decodeText());
        document.getElementById('playBtn').addEventListener('click', () => this.playMorse());
        document.getElementById('copyBtn').addEventListener('click', () => this.copyMorse());
        document.getElementById('clearBtn').addEventListener('click', () => this.clearAll());
        
        document.getElementById('textInput').addEventListener('input', (e) => {
            if (document.getElementById('autoPlayCheck').checked) {
                this.encodeText();
            }
        });
        
        document.getElementById('morseInput').addEventListener('input', (e) => {
            this.decodeText();
        });
        
        document.getElementById('wpmSlider').addEventListener('input', (e) => {
            this.wpm = parseInt(e.target.value);
            document.getElementById('wpmValue').textContent = this.wpm;
            this.audioPlayer.set_wpm(this.wpm);
        });
        
        document.getElementById('volumeSlider').addEventListener('input', (e) => {
            this.volume = parseInt(e.target.value);
            document.getElementById('volumeValue').textContent = this.volume;
        });
        
        document.getElementById('clearHistoryBtn').addEventListener('click', () => this.clearHistory());
        
        document.getElementById('dotBtn').addEventListener('click', () => this.addManualMorse('.'));
        document.getElementById('dashBtn').addEventListener('click', () => this.addManualMorse('-'));
        document.getElementById('spaceBtn').addEventListener('click', () => this.addManualMorse(' '));
        document.getElementById('clearManualBtn').addEventListener('click', () => this.clearManual());
        
        document.addEventListener('keydown', (e) => {
            if (e.target.tagName === 'TEXTAREA') return;
            
            if (e.key === '.' || e.key === 'j' || e.key === 'J') {
                this.addManualMorse('.');
                this.flashButton('dotBtn');
            } else if (e.key === '-' || e.key === 'k' || e.key === 'K') {
                this.addManualMorse('-');
                this.flashButton('dashBtn');
            } else if (e.key === ' ') {
                e.preventDefault();
                this.addManualMorse(' ');
                this.flashButton('spaceBtn');
            }
        });
    }

    setupCanvas() {
        this.canvas = document.getElementById('signalCanvas');
        this.ctx = this.canvas.getContext('2d');
        this.canvas.width = 600;
        this.canvas.height = 200;
    }

    encodeText() {
        const text = document.getElementById('textInput').value;
        if (!text.trim()) return;
        
        const morse = this.morseWave.text_to_morse(text);
        document.getElementById('morseOutput').textContent = morse;
        
        this.addToHistory(text, morse);
    }

    decodeText() {
        const morse = document.getElementById('morseInput').value;
        if (!morse.trim()) return;
        
        if (!this.morseWave.validate_morse(morse)) {
            document.getElementById('textOutput').textContent = 'INVALID MORSE CODE';
            return;
        }
        
        const text = this.morseWave.morse_to_text(morse);
        document.getElementById('textOutput').textContent = text;
    }

    async playMorse() {
        const morse = document.getElementById('morseOutput').textContent;
        if (!morse || this.isPlaying) return;
        
        this.isPlaying = true;
        document.getElementById('playBtn').disabled = true;
        
        try {
            this.audioPlayer.play_morse(morse);
            await this.visualizeMorse(morse);
        } catch (error) {
            console.error('Playback error:', error);
        } finally {
            this.isPlaying = false;
            document.getElementById('playBtn').disabled = false;
        }
    }

    async visualizeMorse(morse) {
        if (!document.getElementById('visualCheck').checked) return;
        
        const led = document.getElementById('ledIndicator');
        const dotDuration = 1200 / this.wpm;
        
        for (let char of morse) {
            if (char === '.') {
                led.classList.add('active');
                this.drawSignal(dotDuration);
                await this.sleep(dotDuration);
                led.classList.remove('active');
                await this.sleep(dotDuration);
            } else if (char === '-') {
                led.classList.add('active');
                this.drawSignal(dotDuration * 3);
                await this.sleep(dotDuration * 3);
                led.classList.remove('active');
                await this.sleep(dotDuration);
            } else if (char === ' ') {
                await this.sleep(dotDuration * 3);
            } else if (char === '/') {
                await this.sleep(dotDuration * 7);
            }
        }
    }

    drawSignal(duration) {
        const width = this.canvas.width;
        const height = this.canvas.height;
        
        this.ctx.fillStyle = 'rgba(10, 14, 39, 0.1)';
        this.ctx.fillRect(0, 0, width, height);
        
        this.ctx.strokeStyle = '#00ff41';
        this.ctx.lineWidth = 2;
        this.ctx.shadowBlur = 10;
        this.ctx.shadowColor = '#00ff41';
        
        const signalHeight = height / 2;
        const pulseWidth = (width * duration) / 1200;
        
        this.ctx.beginPath();
        this.ctx.moveTo(0, signalHeight);
        this.ctx.lineTo(pulseWidth / 4, signalHeight);
        this.ctx.lineTo(pulseWidth / 4, signalHeight - 60);
        this.ctx.lineTo(pulseWidth * 3 / 4, signalHeight - 60);
        this.ctx.lineTo(pulseWidth * 3 / 4, signalHeight);
        this.ctx.lineTo(width, signalHeight);
        this.ctx.stroke();
        
        this.ctx.shadowBlur = 0;
    }

    startIdleAnimation() {
        const animate = () => {
            if (!this.isPlaying) {
                const width = this.canvas.width;
                const height = this.canvas.height;
                
                this.ctx.fillStyle = 'rgba(10, 14, 39, 0.05)';
                this.ctx.fillRect(0, 0, width, height);
                
                this.ctx.strokeStyle = 'rgba(0, 255, 65, 0.3)';
                this.ctx.lineWidth = 1;
                
                const time = Date.now() / 1000;
                this.ctx.beginPath();
                for (let x = 0; x < width; x++) {
                    const y = height / 2 + Math.sin(x * 0.02 + time) * 20;
                    if (x === 0) {
                        this.ctx.moveTo(x, y);
                    } else {
                        this.ctx.lineTo(x, y);
                    }
                }
                this.ctx.stroke();
            }
            
            this.animationId = requestAnimationFrame(animate);
        };
        
        animate();
    }

    copyMorse() {
        const morse = document.getElementById('morseOutput').textContent;
        if (!morse) return;
        
        navigator.clipboard.writeText(morse).then(() => {
            const btn = document.getElementById('copyBtn');
            const originalText = btn.querySelector('span').textContent;
            btn.querySelector('span').textContent = '✓ COPIED';
            setTimeout(() => {
                btn.querySelector('span').textContent = originalText;
            }, 2000);
        });
    }

    clearAll() {
        document.getElementById('textInput').value = '';
        document.getElementById('morseInput').value = '';
        document.getElementById('morseOutput').textContent = '';
        document.getElementById('textOutput').textContent = '';
    }

    addManualMorse(symbol) {
        this.manualMorse += symbol;
        document.getElementById('manualMorse').textContent = this.manualMorse;
        
        if (this.morseWave.validate_morse(this.manualMorse)) {
            const text = this.morseWave.morse_to_text(this.manualMorse);
            document.getElementById('manualText').textContent = text;
        }
        
        const led = document.getElementById('ledIndicator');
        led.classList.add('active');
        setTimeout(() => led.classList.remove('active'), 100);
    }

    clearManual() {
        this.manualMorse = '';
        document.getElementById('manualMorse').textContent = '';
        document.getElementById('manualText').textContent = '';
    }

    flashButton(btnId) {
        const btn = document.getElementById(btnId);
        btn.style.background = 'rgba(0, 255, 65, 0.3)';
        setTimeout(() => {
            btn.style.background = '';
        }, 150);
    }

    addToHistory(text, morse) {
        const item = {
            text,
            morse,
            timestamp: Date.now()
        };
        
        this.history.unshift(item);
        if (this.history.length > 50) {
            this.history.pop();
        }
        
        this.saveHistory();
        this.renderHistory();
    }

    renderHistory() {
        const historyList = document.getElementById('historyList');
        historyList.innerHTML = '';
        
        this.history.forEach(item => {
            const div = document.createElement('div');
            div.className = 'history-item';
            
            const time = new Date(item.timestamp).toLocaleTimeString();
            div.innerHTML = `
                <div class="history-time">${time}</div>
                <div class="history-text">${item.text}</div>
                <div class="history-morse">${item.morse}</div>
            `;
            
            div.addEventListener('click', () => {
                document.getElementById('textInput').value = item.text;
                document.getElementById('morseOutput').textContent = item.morse;
            });
            
            historyList.appendChild(div);
        });
    }

    saveHistory() {
        localStorage.setItem('morsewave_history', JSON.stringify(this.history));
    }

    loadHistory() {
        const saved = localStorage.getItem('morsewave_history');
        if (saved) {
            this.history = JSON.parse(saved);
            this.renderHistory();
        }
    }

    clearHistory() {
        this.history = [];
        localStorage.removeItem('morsewave_history');
        this.renderHistory();
    }

    generateMorseTable() {
        const table = document.getElementById('morseTable');
        const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
        
        for (let char of chars) {
            const morse = this.morseWave.text_to_morse(char);
            const div = document.createElement('div');
            div.className = 'morse-item';
            div.innerHTML = `
                <span class="morse-char">${char}</span>
                <span class="morse-code">${morse}</span>
            `;
            table.appendChild(div);
        }
    }

    sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
}

const app = new MorseWaveApp();
app.init().catch(console.error);