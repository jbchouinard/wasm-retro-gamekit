import { memory } from "wasm-game-of-life/wasm_game_of_life_bg"
import { game_of_life, fake_game, WindowHandle, init_once } from "wasm-game-of-life";

init_once();

// const gameHandle = game_of_life(200, 200, 0.5);
const gameHandle = fake_game(200, 200);
const windowHandle = WindowHandle.new(gameHandle);
const canvas = document.getElementById("game-of-life-canvas");
const canvasContext = canvas.getContext("2d");

var imageWidth;
var imageHeight;
var imageDataPtr;
var imageDataSize;
var imageDataArray;
var imageData;

function onWindowResize() {
    const width = window.innerWidth;
    const height = window.innerHeight;

    windowHandle.set_screen_size(width - 10, height - 10);
    imageWidth = windowHandle.image_width();
    imageHeight = windowHandle.image_height();
    imageDataPtr = windowHandle.image_data();
    imageDataSize = windowHandle.image_data_size();
    imageDataArray = new Uint8ClampedArray(memory.buffer, imageDataPtr, imageDataSize);
    imageData = new ImageData(imageDataArray, imageWidth, imageHeight);

    canvas.width = imageWidth;
    canvas.height = imageHeight;
}

onWindowResize();
addEventListener("resize", onWindowResize);

const countFps = true;
const countFpsInterval = 10;
var countFpsTicks = 0;
var countFpsStart = window.performance.now();
var countFpsAvg = 0;

function renderLoop() {
    gameHandle.tick();
    gameHandle.render(windowHandle);
    canvasContext.putImageData(imageData, 0, 0);
    requestAnimationFrame(renderLoop);

    if (countFps) {
        countFpsTicks++;
        if (countFpsTicks == countFpsInterval) {
            let now = window.performance.now();
            let elapsed = now - countFpsStart;
            countFpsAvg = Math.round(countFpsInterval * 1000 / elapsed);
            countFpsStart = now;
            countFpsTicks = 0;
        }
        canvasContext.strokeStyle = "red";
        canvasContext.fillStyle = "black";
        canvasContext.strokeText(`${countFpsAvg}`, 10, 20);
        canvasContext.fillText(`${countFpsAvg}`, 10, 20);
    }
}

requestAnimationFrame(renderLoop);
