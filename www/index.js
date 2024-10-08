//import { greet } from './portfolio.js';   //development path is ../pkg/portfolio.js
import { initKarplusStrong, playKarplusStrong } from './karplusStrong.js';
import { initGranular, playGranularSynth } from './granular.js';

import init from './portfolio.js'

function setupSectionScrolling() {
    const contentContainer = document.getElementById('content-container');
    const sections = document.querySelectorAll('.section');
    let currentSectionIndex = 0;

    function scrollToSection(index) {
        sections[index].scrollIntoView({ behavior: 'smooth' });
    }

    function handleWheel(event) {
        event.preventDefault();
        if (event.deltaY > 0 && currentSectionIndex < sections.length - 1) {
            currentSectionIndex++;
        } else if (event.deltaY < 0 && currentSectionIndex > 0) {
            currentSectionIndex--;
        }
        scrollToSection(currentSectionIndex);
    }

    contentContainer.addEventListener('wheel', handleWheel, { passive: false });
}

function workClick() {
    const work = document.getElementsByClassName('work');
    for (let i = 0; i < work.length; i++) {
        work[i].addEventListener('click', function() {
            const description = this.children[1];
            if (description.style.display === 'block') {
                description.style.display = 'none';
            } else {
                description.style.display = 'block';
            }
        });
    }
}
        

async function run() {
    await init();
    await initKarplusStrong();
    await initGranular();
    setupSectionScrolling();
    workClick();

    const triggerSynth = document.getElementById('trigger-synth-button');
    const triggerGranularSynth = document.getElementById('trigger-granular-synth-button');
    const fileUploadGrain = document.getElementById('decay-input');
    const decayInput = document.getElementById('decay-input');


    if(triggerSynth) {
        triggerSynth.addEventListener('click', () => {
            playKarplusStrong(440, 2); // Play A4 (440 Hz) for 2 seconds
        });
    }
    if(decayInput) {
        decayInput.addEventListener('input', () => {
            const decay = parseFloat(decayInput.value);
            setDecay(decay);
            console.log('Decay:', decay);
        });
    }
    if(triggerGranularSynth){
        triggerGranularSynth.addEventListener('click', async () => {
            const fileInput = document.getElementById('file-input');
            const file = fileInput.files[0]; // Get the first file from the input
    
            if (file) {
                const arrayBuffer = await file.arrayBuffer(); // Read the file as an ArrayBuffer
                // Here you can process the ArrayBuffer as needed for your granular synth
                
                const audioContext = new window.AudioContext;

                audioContext.decodeAudioData(arrayBuffer, (audioBuffer) => {
                    // Get the audio data as a Float32Array
                    const audioData = audioBuffer.getChannelData(0); // Assuming mono audio, use channel 0
    
                    // Now you can use the audioData with your granular synth
                    playGranularSynth(audioData, 50); // Pass the Float32Array to your synth
                    console.log('File uploaded:', file.name);
                    console.log('Audio Data:', audioData);
                }, (error) => {
                    console.error('Error decoding audio data:', error);
                });    
            } else {
                console.error('No file selected.');
            }
        })
    }

    
}

run();
