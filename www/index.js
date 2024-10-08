//import { greet } from './portfolio.js';   //development path is ../pkg/portfolio.js
import { initAudio, playKarplusStrong } from './karplusStrong.js';

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
    await initAudio();
    setupSectionScrolling();
    workClick();

    const triggerSynth = document.getElementById('trigger-synth-button');
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
    
}

run();
