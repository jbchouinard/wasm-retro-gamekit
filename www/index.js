import { memory } from "wasm-game-of-life/wasm_game_of_life_bg"
import { BouncyBox, init_once } from "wasm-game-of-life";

init_once();

const canvas = document.getElementById("game-canvas");
const context2d = canvas.getContext("2d");

const renderScale = 1;
const renderMargin = 5;
const coeffRestitution = 0.9;
const game = BouncyBox(
    (window.innerWidth - renderMargin) / renderScale,
    (window.innerHeight - renderMargin) / renderScale,
    coeffRestitution
);
const eventQueue = game.event_queue();

var requireRedraw = true;

window.addEventListener("keydown", onKeyDown)
window.addEventListener("keyup", onKeyUp);
window.addEventListener("resize", onResize);

onResize();

requestAnimationFrame(gameLoop);

console.time("gameLoop");

function gameLoop() {
    let response = game.tick(window.performance.now());
    if (response == "Finished") {
        return
    } else if (response == "RequestRedraw" || requireRedraw) {
        const gameWindow = game.window();
        let width = gameWindow.image_width();
        let height = gameWindow.image_height();
        canvas.width = width;
        canvas.height = height;
        const imageDataArray = new Uint8ClampedArray(
            memory.buffer,
            gameWindow.image_data_ptr(),
            gameWindow.image_data_size(),
        );
        const imageData = new ImageData(imageDataArray, width, height);
        context2d.putImageData(imageData, 0, 0);
        requireRedraw = false;
        console.timeEnd("gameLoop");
        console.time("gameLoop");
    }
    requestAnimationFrame(gameLoop);
}

function onResize() {
    eventQueue.send_window_resize(window.innerWidth, window.innerHeight);
    requireRedraw = true;
}

function onKeyUp(event) {
    eventQueue.send_key_up(event.key, event.altKey, event.ctrlKey, event.shiftKey, event.metaKey);
}

function onKeyDown(event) {
    eventQueue.send_key_down(event.key, event.altKey, event.ctrlKey, event.shiftKey, event.metaKey);
}
