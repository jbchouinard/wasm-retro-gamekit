import { memory } from "wasm-retro-gamekit-life/wasm_retro_gamekit_life_bg.wasm"
import { GameOfLife, init_once } from "wasm-retro-gamekit-life";
import { startGameFullscreen } from "retro-gamekit-bootstrap";

init_once();

startGameFullscreen(
    memory,
    GameOfLife(window.innerWidth / 2, window.innerHeight / 2, 0.5, 1000),
    document.getElementById("game-canvas"),
);
