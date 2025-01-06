//import { greet } from '../pkg/portfolio.js';   //development path is ../pkg/portfolio.js
import { initAudio, playKarplusStrong } from './karplusStrong.js';
import { startGame } from './game.js';

function setupSectionScrolling(gameEnabled = false) {
    const contentContainer = document.getElementById('content-container');
    const sections = document.querySelectorAll('.section');
    let currentSectionIndex = 0;

    /**
     * Scroll to the section with the given index.
     * @param {number} index - The index of the section to scroll to.
     */
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

    if(!gameEnabled) {
        console.log("enabled scroll");
        //contentContainer.addEventListener('wheel', handleWheel, { passive: false });
    } else {
        console.log("disabled scroll");
        contentContainer.removeEventListener('wheel', handleWheel);
        
    }

    
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
    //setupSectionScrolling(false);
    workClick();

    const triggerSynth = document.getElementById('trigger-synth-button');
    const decayInput = document.getElementById('decay-input');
    const startGameButton = document.getElementById('start-game-button');

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
    
    if(startGameButton) {
        startGameButton.addEventListener('click', async () => {
            await document.getElementById('game-window').requestPointerLock();

            const mouseGlow = document.querySelector('.mouse-glow');
            mouseGlow.style.display = 'none';
            // Assuming you have an element with the class 'myClass'
            var element = document.querySelector('.content');

            // Remove the class 'myClass'
            element.classList.remove('content');

            
            setupSectionScrolling(true);
            startGame();
        });
    }
}

run();
