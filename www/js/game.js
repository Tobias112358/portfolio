import { build_game } from '../pkg/portfolio.js';

const startGame = async () => {
    build_game("#game-window");
}

export { startGame };