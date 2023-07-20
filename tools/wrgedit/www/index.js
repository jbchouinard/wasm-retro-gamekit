import { memory } from "wrg-edit/wrg_edit_bg.wasm"
import { WrgEditorHandle, init_once } from "wrg-edit";
import { Game } from "retro-gamekit-bootstrap";


init_once();

const gameHandle = WrgEditorHandle();
const game = new Game(gameHandle, memory, "wrg").fullscreen().withFileInput("openImage", "o", 16, ".wrg");
game.start();
