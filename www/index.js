import init, { greet } from '../pkg/portfolio.js';

async function run() {
    await init();

    const nameInput = document.getElementById('name-input');
    const greetButton = document.getElementById('greet-button');
    const greetingElement = document.getElementById('greeting');

    greetButton.addEventListener('click', () => {
        const name = nameInput.value || 'World';
        const greeting = greet(name);
        greetingElement.textContent = greeting;
    });
}

run();
