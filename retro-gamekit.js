export class Game {
    #gameHandle;
    #eventQueue;
    #memory;
    #rootElement;
    #canvasElement;
    #renderContext;
    #fpsCounter;
    #resizeObserver;
    #logFps = false;
    #finished = false;
    #listeners = [];

    constructor(gameHandle, memory, rootElement) {
        this.#gameHandle = gameHandle;
        this.#memory = memory;
        this.#eventQueue = gameHandle.event_queue();
        this.#rootElement = getElement(rootElement);
        this.#rootElement.tabIndex = 1;
        this.#canvasElement = document.createElement("canvas");
        this.#renderContext = this.#canvasElement.getContext("2d");
        this.#rootElement.appendChild(this.#canvasElement);
    }
    fullscreen() {
        this.#resizeObserver = new ResizeObserver((entries) => {
            const entry = entries[0];
            const width = Math.floor(entry.contentRect.width);
            const height = Math.floor(entry.contentRect.height);
            this.#eventQueue.send_window_resize(width, height);
        });
        this.#resizeObserver.observe(this.#rootElement);
        return this;
    }
    windowed(width, height) {
        this.#eventQueue.send_window_resize(width, height);
        return this;
    }
    withKeyboard() {
        this.#listeners = this.#listeners.concat(keyboardEventQueueListeners(this.#rootElement, this.#eventQueue));
        return this;
    }
    withMouse() {
        this.#listeners = this.#listeners.concat(mouseEventQueueListeners(this.#canvasElement, this.#eventQueue));
        return this;
    }
    withFileInput(name, bindKey = null, maxSizeMB = 8, accept = null) {
        const inputElement = document.createElement("input");
        inputElement.type = "file";
        inputElement.hidden = true;
        if (accept != null) {
            inputElement.accept = accept;
        }

        this.#rootElement.appendChild(inputElement);
        this.#listeners = this.#listeners.concat(
            fileInputListeners(this.#rootElement, inputElement, this.#eventQueue, name, bindKey, maxSizeMB)
        );
        return this;
    }
    finish() {
        this.#finished = true;
        this.#listeners.forEach((listener) => { listener.remove(); });
        if (this.#resizeObserver != null) {
            this.#resizeObserver.disconnect();
        }
        this.#listeners = [];
    }
    start() {
        this.#listeners.forEach((listener) => { listener.add(); });

        const gameHandle = this.#gameHandle;
        const canvas = this.#canvasElement;
        const context2d = this.#renderContext;
        const memory = this.#memory;
        const gameLoop = () => {
            if (this.#finished) {
                return;
            }
            let response = gameHandle.tick(now());
            if (response == "Finished") {
                this.finish();
            } else if (response == "RequestRedraw") {
                const gameWindow = gameHandle.window();
                let displayWidth = gameWindow.image_width();
                let displayHeight = gameWindow.image_height();
                if (canvas.width != displayWidth) {
                    canvas.width = displayWidth;
                }
                if (canvas.height != displayHeight) {
                    canvas.height = displayHeight;
                }
                const imageDataArray = new Uint8ClampedArray(
                    memory.buffer,
                    gameWindow.image_data_ptr(),
                    gameWindow.image_data_size(),
                );
                const imageData = new ImageData(imageDataArray, displayWidth, displayHeight);
                context2d.putImageData(imageData, 0, 0);
                if (this.#logFps) {
                    this.#getFpsCounter().increment(now());
                }
            }
            requestAnimationFrame(gameLoop);
        }
        requestAnimationFrame(gameLoop);
        return this;
    }
    #getFpsCounter() {
        if (this.#fpsCounter == null) {
            this.#fpsCounter = new FpsCounter(1000);
        }
        return this.#fpsCounter;
    }
    logFps() {
        this.#logFps = true;
        return this;
    }
}

function getElement(elementOrElementId) {
    if (typeof elementOrElementId === "string") {
        return document.getElementById(elementOrElementId);
    } else {
        return elementOrElementId;
    }
}

class Listener {
    #element;
    #type;
    #func;

    constructor(element, type, func) {
        this.#element = element;
        this.#type = type;
        this.#func = func;
    }
    add() {
        this.#element.addEventListener(this.#type, this.#func);
    }
    remove() {
        this.#element.removeEventListener(this.#type, this.#func);
    }
}

function mouseEventQueueListeners(element, eventQueue) {
    return [
        new Listener(element, "mousedown", (e) => {
            eventQueue.send_mouse_button(now(), e.layerX / element.width, e.layerY / element.height, e.button, false);
        }),
        new Listener(element, "mouseup", (e) => {
            eventQueue.send_mouse_button(now(), e.layerX / element.width, e.layerY / element.height, e.button, true);
        }),
        new Listener(element, "mousemove", (e) => {
            eventQueue.send_mouse_move(now(), e.layerX / element.width, e.layerY / element.height);
        }),
        new Listener(element, "contextmenu", (e) => { e.preventDefault(); })
    ];
}

function keyboardEventQueueListeners(element, eventQueue) {
    return [
        new Listener(element, "keydown", (e) => {
            if (!e.repeat) {
                eventQueue.send_key_down(e.keyCode, now());
            }
            e.preventDefault();
        }),
        new Listener(element, "keyup", (e) => {
            eventQueue.send_key_up(e.keyCode, now());
            e.preventDefault();
        })
    ];
}

function fileInputListeners(rootElement, inputElement, eventQueue, name, bindKey = null, maxSizeMB = 8) {
    let listeners = [];

    if (bindKey != null) {
        listeners.push(new Listener(rootElement, "keydown", (e) => {
            if (e.key === bindKey) {
                inputElement.click();
            }
        }));
    }

    listeners.push(new Listener(inputElement, "change", () => {
        for (let i = 0; i < inputElement.files.length; i++) {
            const file = inputElement.files[i];
            const reader = new FileReader();
            reader.addEventListener("loadend", (event) => {
                if (event.target.result != null) {
                    const arr = new Uint8Array(event.target.result);
                    if (arr.length <= (maxSizeMB * 1024 * 1024)) {
                        eventQueue.send_file_read(String(name), String(file.name), arr);
                    } else {
                        alert(`File too large - max size is ${maxSizeMB}MB.`);
                    }
                }
            })
            reader.readAsArrayBuffer(file);
        }
    }));

    return listeners;
}

function FpsCounter(logIntervallMilliseconds) {
    return {
        frameCount: 0,
        startTs: 0,
        lastFrameTs: 0,
        increment: function (ts) {
            this.frameCount++;
            this.lastFrameTs = ts;
            if ((ts - this.startTs) >= logIntervallMilliseconds) {
                this.log();
                this.frameCount = 0;
                this.startTs = ts;
            }
        },
        log: function () {
            if (this.frameCount === 0) {
                return;
            }
            const totalTime = this.lastFrameTs - this.startTs;
            const msPerFrame = totalTime / this.frameCount;
            const framePerSec = 1000 / msPerFrame;
            console.log(`${Math.round(framePerSec)} fps (${Math.round(msPerFrame * 10) / 10}ms per frame)`);
        }
    };
}

function now() {
    return window.performance.now();
}
