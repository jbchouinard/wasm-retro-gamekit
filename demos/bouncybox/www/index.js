import { memory } from "wasm_retro_gamekit_bouncybox/wasm_retro_gamekit_bouncybox_bg.wasm"
import { BouncyBox, init_once } from "wasm_retro_gamekit_bouncybox";
import { startGameFullscreen } from "retro-gamekit-bootstrap";

init_once();

startGameFullscreen(
    memory,
    BouncyBox(window.innerWidth, window.innerHeight, 0.8),
    document.getElementById("game-canvas"),
);
