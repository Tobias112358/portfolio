import { KarplusStrong } from './portfolio.js';

let audioContext;

/**
 * Initializes the audio context and loads the WebAssembly module.
 * This function must be called before any audio playback can occur.
 * 
 * @async
 * @function initKarplusStrong
 * @returns {Promise<void>} A promise that resolves when the audio context is initialized and the WebAssembly module is loaded.
 */
async function initKarplusStrong() {
    audioContext = new (window.AudioContext || window.webkitAudioContext)();
}

/**
 * Plays a Karplus-Strong synthesized sound at the specified frequency for the given duration.
 * 
 * @function playKarplusStrong
 * @param {number} frequency - The frequency of the note to play, in Hertz.
 * @param {number} duration - The duration of the note, in seconds.
 */

function playKarplusStrong(frequency, duration) {
    const sampleRate = audioContext.sampleRate;
    const synth = new KarplusStrong(frequency, sampleRate);
    const bufferSize = Math.floor(sampleRate * duration);
    const buffer = audioContext.createBuffer(1, bufferSize, sampleRate);
    const channelData = buffer.getChannelData(0);

    for (let i = 0; i < bufferSize; i++) {
        channelData[i] = synth.tick();
    }

    const source = audioContext.createBufferSource();
    source.buffer = buffer;
    source.connect(audioContext.destination);
    source.start();
}

export { initKarplusStrong, playKarplusStrong };