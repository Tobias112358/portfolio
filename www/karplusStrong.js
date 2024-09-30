import init, { KarplusStrong } from './pkg/portfolio.js';

let audioContext;

async function initAudio() {
    await init();
    audioContext = new (window.AudioContext || window.webkitAudioContext)();
}

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

export { initAudio, playKarplusStrong };