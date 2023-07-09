import { GameOfLife, init_once } from "wasm-game-of-life";

init_once();


const GAME_WIDTH = 400;
const GAME_HEIGHT = 300;
const LIFE_DENSITY = 0.5;
const PIXEL_SCALE = 3;
const IMAGE_WIDTH = GAME_WIDTH * PIXEL_SCALE;
const IMAGE_HEIGHT = GAME_HEIGHT * PIXEL_SCALE;

const gameOfLife = GameOfLife.conway(GAME_WIDTH, GAME_HEIGHT, LIFE_DENSITY);


const canvas = document.getElementById("game-of-life-canvas");
canvas.height = IMAGE_HEIGHT;
canvas.width = IMAGE_WIDTH;

const ctx = canvas.getContext("2d");


function renderLoop() {
    const imageData = ctx.createImageData(IMAGE_WIDTH, IMAGE_HEIGHT);
    gameOfLife.paint(imageData.data, PIXEL_SCALE);
    ctx.putImageData(imageData, 0, 0);
    gameOfLife.tick();
}

setInterval(renderLoop, 200);
