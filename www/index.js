//import { greet } from './portfolio.js';   //development path is ../pkg/portfolio.js
import { initAudio, playKarplusStrong } from './karplusStrong.js';


async function run() {
    await initAudio();

    const triggerSynth = document.getElementById('trigger-synth-button');
    const decayInput = document.getElementById('decay-input');

    triggerSynth.addEventListener('click', () => {
        playKarplusStrong(440, 2); // Play A4 (440 Hz) for 2 seconds
            });
    decayInput.addEventListener('input', () => {
        const decay = parseFloat(decayInput.value);
        setDecay(decay);
        console.log('Decay:', decay);
    });
    
}

run();
