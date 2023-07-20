import { memory } from "wasm-retro-gamekit-life/wasm_retro_gamekit_life_bg.wasm"
import { GameOfLife, init_once } from "wasm-retro-gamekit-life";
import { Game } from "retro-gamekit";

init_once();

const gameHandle = GameOfLife(128, 128, 0.5, 1000);
const game = new Game(gameHandle, memory, "wrg").windowed(512, 512);
game.start();
