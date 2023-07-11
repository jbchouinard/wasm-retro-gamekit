import { memory } from "wasm-game-of-life/wasm_game_of_life_bg"
import { make_game_of_life, make_input, make_game_window, init_once } from "wasm-game-of-life";

init_once();

var countFps = false;
const countFpsInterval = 30;

const canvas = document.getElementById("game-canvas");
const canvasContext = canvas.getContext("2d");

const inputHandle = make_input();

window.addEventListener("keydown", (event) => {
    if (event.key == "F") {
        countFps = !countFps;
    }
    if (event.key == "?") {
        alert(`Key Bindings\n${inputHandle.key_map_help()}`);
    }
    inputHandle.key_down(event.key);
});
window.addEventListener("keyup", (event) => {
    inputHandle.key_up(event.key);
});

runGame(make_game_of_life(inputHandle, 100, 100, 0.5, 1000));

function runGame(gameHandle) {
    const gameWindowHandle = make_game_window(gameHandle);
    let requireRedraw = true;

    function onResize() {
        const width = window.innerWidth;
        const height = window.innerHeight;


        gameWindowHandle.set_screen_size(width, height);
        canvas.height = gameWindowHandle.image_height();
        canvas.width = gameWindowHandle.image_width();

        requireRedraw = true;
        console.log(`resized window to ${width}x${height}, canvas size ${canvas.width}x${canvas.height}`);
    }

    onResize();
    addEventListener("resize", onResize);

    let countFpsTicks = 0;
    let countFpsStart = window.performance.now();

    function gameLoop() {
        let signal = gameHandle.tick(window.performance.now(), inputHandle);
        if (signal == "RequestRedraw") {
            requireRedraw = true;
        } else if (signal == "Finished") {
            return
        }

        if (requireRedraw) {
            gameHandle.render(gameWindowHandle);
            canvasContext.putImageData(getImageData(gameWindowHandle), 0, 0);
            requireRedraw = false;
        }
        requestAnimationFrame(gameLoop);

        if (countFps) {
            countFpsTicks++;
            if (countFpsTicks == countFpsInterval) {
                const now = window.performance.now();
                const elapsed = now - countFpsStart;
                const countFpsAvg = Math.round(countFpsInterval * 1000 / elapsed);
                countFpsStart = now;
                countFpsTicks = 0;
                console.log(`${countFpsAvg} FPS`)
            }
        }
    }
    requestAnimationFrame(gameLoop);
}

function getImageData(gameWindowHandle) {
    const imageDataArray = new Uint8ClampedArray(
        memory.buffer,
        gameWindowHandle.image_data(),
        gameWindowHandle.image_data_size(),
    );
    return new ImageData(
        imageDataArray,
        gameWindowHandle.image_width(),
        gameWindowHandle.image_height()
    );
}
