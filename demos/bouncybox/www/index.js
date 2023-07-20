import { memory } from "wasm-retro-gamekit-bouncybox/wasm_retro_gamekit_bouncybox_bg.wasm"
import { BouncyBox, init_once } from "wasm-retro-gamekit-bouncybox";
import { Game } from "retro-gamekit";

init_once();


const gameHandle = BouncyBox(window.innerWidth, window.innerHeight, 0.8);
const game = new Game(gameHandle, memory, "wrg").fullscreen().withMouse().withKeyboard();
game.start();
