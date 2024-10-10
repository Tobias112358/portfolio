import { GranularSynth } from './portfolio.js';

let audioContext;

/**
 * Initializes the audio context and loads the WebAssembly module.
 * This function must be called before any audio playback can occur.
 * 
 * @async
 * @function initAudio
 * @returns {Promise<void>} A promise that resolves when the audio context is initialized and the WebAssembly module is loaded.
 */
async function initGranular() {
    audioContext = new (window.AudioContext || window.webkitAudioContext)();
}

/**
 * Plays a granular synthesized sound using the provided sample file buffer for the given duration.
 * 
 * @function playGranularSynth
 * 
 * @param {ArrayBuffer} sampleFileBuffer - The audio sample file buffer to use for granular synthesis.
 * @param {number} duration - The duration of the sound, in seconds.
 */

function playGranularSynth(sampleFileBuffer, duration, grainArray) {
    const sampleRate = audioContext.sampleRate;
    const audioData = new Float32Array(sampleFileBuffer);
    const synth = new GranularSynth(sampleRate);
    synth.load_sample(audioData);
    const bufferSize = Math.floor(sampleRate * duration);
    const buffer = audioContext.createBuffer(1, bufferSize, sampleRate);
    const channelData = buffer.getChannelData(0);

    for(let i = 0; i < grainArray.length; i = i+2){
        synth.add_grain(grainArray[i], grainArray[i+1] - grainArray[i]);
    }
    const generatedSamples = synth.generate();

    // Set the channel data to the generated samples
    for (let i = 0; i < Math.min(generatedSamples.length, bufferSize); i++) {
        channelData[i] = generatedSamples[i]; // Copy generated samples to channel data
    }

    const source = audioContext.createBufferSource();
    source.buffer = buffer;
    source.connect(audioContext.destination);
    source.start();
}

export { initGranular, playGranularSynth };