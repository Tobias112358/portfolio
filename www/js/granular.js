import { GranularSynth } from './portfolio.js';

let audioContext;
let synth; // Declare synth at a higher scope

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

async function playGranularSynth(sampleFileBuffer, duration, grainArray) {
    const sampleRate = audioContext.sampleRate;
    const audioData = new Float32Array(sampleFileBuffer);
    synth = new GranularSynth(sampleRate, 44100); // Buffer size of 1 second at 44.1kHz
    synth.load_sample(audioData);

    // Initial grain setup
    for (let i = 0; i < grainArray.length; i += 2) {
        synth.add_grain(grainArray[i], grainArray[i + 1] - grainArray[i]);
    }

    // Start the playback loop
    const loop = () => {
        if (synth.is_playing()) {
            const generatedSamples = synth.generate(); // Generate samples
            const bufferData = synth.read_buffer(); // Read from the circular buffer

            // Create an AudioBufferSourceNode for playback
            const audioBuffer = audioContext.createBuffer(1, bufferData.length, sampleRate);
            audioBuffer.copyToChannel(bufferData, 0); // Copy data to the audio buffer

            const source = audioContext.createBufferSource();
            source.buffer = audioBuffer;
            source.connect(audioContext.destination);
            source.start(0); // Start immediately

            // Optionally, you can stop the source after a certain duration
            source.stop(audioContext.currentTime + 1); // Play for 1 second
        }
        requestAnimationFrame(loop); // Continue the loop
    };

    synth.play(); // Start playback
    loop(); // Start the loop
}

// Function to stop playback
function stopGranularSynth() {
    if (synth) {
        synth.stop(); // Stop the synth
    }
}

export { initGranular, playGranularSynth, stopGranularSynth };
