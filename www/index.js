//import { greet } from './portfolio.js';   //development path is ../pkg/portfolio.js
import { initAudio, playKarplusStrong } from './karplusStrong.js';


async function run() {
    await initAudio();

    const nameInput = document.getElementById('name-input');
    const greetButton = document.getElementById('greet-button');
    const greetingElement = document.getElementById('greeting');
    const triggerSynth = document.getElementById('trigger-synth-button');

    greetButton.addEventListener('click', () => {
        const name = nameInput.value || 'World';
        //const greeting = greet(name);
        greetingElement.textContent = greeting;
    });

    triggerSynth.addEventListener('click', () => {
        playKarplusStrong(440, 2); // Play A4 (440 Hz) for 2 seconds
            });
            
}

run();
