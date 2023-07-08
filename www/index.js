import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new(100, 50, 50);

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();
}
renderLoop();
const timer = setInterval(renderLoop, 100);

