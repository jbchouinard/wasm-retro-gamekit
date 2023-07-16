import { memory } from "wasm_retro_gamekit_life/wasm_retro_gamekit_life_bg"
import { GameOfLife, init_once } from "wasm_retro_gamekit_life";
import { startGameFullscreen } from "retro-gamekit-bootstrap";

init_once();

startGameFullscreen(
    memory,
    GameOfLife(window.innerWidth / 2, window.innerHeight / 2, 0.5, 1000),
    document.getElementById("game-canvas"),
    "frametime"
);
