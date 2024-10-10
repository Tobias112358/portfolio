//import { greet } from './portfolio.js';   //development path is ../pkg/portfolio.js
import { initKarplusStrong, playKarplusStrong } from './karplusStrong.js';
import { initGranular, playGranularSynth } from './granular.js';

import init from './portfolio.js'

let audioDataLength; // Global variable to store the length of the audio data
let sampleRate;
let grains = [0,0];

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

function drawWaveform(audioData) {
    const canvas = document.getElementById('waveform-canvas');

    canvas.addEventListener('click', (event) => {
        const rect = canvas.getBoundingClientRect();
        const x = event.clientX - rect.left; // Get the x position relative to the canvas
        const width = canvas.width;
    
        // Calculate the timestamp based on the click position
        const timestamp = (x / width) * audioDataLength / sampleRate; // Convert to seconds
    
        console.log('Selected timestamp:', timestamp); // Log the selected timestamp
        // You can also display this timestamp in the UI if desired
        let timestampDisplayID = 'timestamp-display-start';
        const tsSelector = document.getElementById("timestamp-selector");
        if(tsSelector.checked == true) {
            timestampDisplayID = 'timestamp-display-end';
        }
        const timestampDisplay = document.getElementById(timestampDisplayID);

        if(timestampDisplayID == 'timestamp-display-start'){
            grains[0] = timestamp.toFixed(2);
        } else {
            grains[1] = timestamp.toFixed(2);
        }
        if (timestampDisplay) {
            timestampDisplay.textContent = `Selected timestamp: ${timestamp.toFixed(2)} seconds`;
        }
    });


    const ctx = canvas.getContext('2d');
    const width = canvas.width;
    const height = canvas.height;

    // Store the length of the audio data
    audioDataLength = audioData.length;

    // Clear the canvas
    ctx.clearRect(0, 0, width, height);

    // Draw the waveform
    ctx.beginPath();
    const sliceWidth = width / audioData.length;
    let x = 0;

    for (let i = 0; i < audioData.length; i++) {
        const v = audioData[i] * 0.5; // Scale the value to fit in the canvas
        const y = (1 + v) * height / 2; // Invert the y-axis
        if (i === 0) {
            ctx.moveTo(x, y);
        } else {
            ctx.lineTo(x, y);
        }
        x += sliceWidth;
    }

    ctx.lineTo(width, height / 2);
    ctx.strokeStyle = 'blue'; // Set the color of the waveform
    ctx.stroke();
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
    const fileInput = document.getElementById('file-input');

    if(fileInput){
        fileInput.addEventListener('change', async (e) => {
            const file = e.target.files[0];
            if (file) {
                const audioContext = new AudioContext();
                const audioBuffer = await file.arrayBuffer();
                audioContext.decodeAudioData(audioBuffer, (buffer) => {
                    
                    const audioData = buffer.getChannelData(0); // Assuming mono audio, use channel 0
                    // Draw the waveform
                    drawWaveform(audioData);
                    
                    sampleRate = audioContext.sampleRate;

                    console.log('Audio file loaded successfully:', buffer);
                }, (error) => {
                    console.error('Error loading audio file:', error);
                });
            }
        });
    }


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
            const file = fileInput.files[0]; // Get the first file from the input
    
            if (file) {
                const arrayBuffer = await file.arrayBuffer(); // Read the file as an ArrayBuffer
                // Here you can process the ArrayBuffer as needed for your granular synth
                
                const audioContext = new window.AudioContext;

                audioContext.decodeAudioData(arrayBuffer, (audioBuffer) => {
                    // Get the audio data as a Float32Array
                    const audioData = audioBuffer.getChannelData(0); // Assuming mono audio, use channel 0

                    // Now you can use the audioData with your granular synth
                    playGranularSynth(audioData, 50, grains); // Pass the Float32Array to your synth
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
