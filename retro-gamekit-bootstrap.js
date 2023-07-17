export function startGameFullscreen(memory, gameHandle, canvas, consoleFrameTimer) {
    const eventQueue = gameHandle.event_queue();
    // Set initial window size
    eventQueue.send_window_resize(window.innerWidth, window.innerHeight);
    addWindowResizeListener(eventQueue);
    addInputListeners(eventQueue, canvas, window);

    startGameLoop(memory, gameHandle, canvas, consoleFrameTimer)
}

var t = window.performance.now();

function startGameLoop(memory, gameHandle, canvas, consoleFrameTimer) {
    if (consoleFrameTimer) {
        consoleFrameTimer = String(consoleFrameTimer);
        console.time(consoleFrameTimer);
    }

    let requireRedraw = true;
    let context2d = canvas.getContext("2d");

    function gameLoop() {
        t = window.performance.now();
        let response = gameHandle.tick(t);
        if (response == "Finished") {
            return
        } else if (response == "RequestRedraw" || requireRedraw) {
            const gameWindow = gameHandle.window();
            let width = gameWindow.image_width();
            let height = gameWindow.image_height();
            if (canvas.width != width) {
                canvas.width = width;
            }
            if (canvas.height != height) {
                canvas.height = height;
            }
            const imageDataArray = new Uint8ClampedArray(
                memory.buffer,
                gameWindow.image_data_ptr(),
                gameWindow.image_data_size(),
            );
            const imageData = new ImageData(imageDataArray, width, height);
            context2d.putImageData(imageData, 0, 0);
            requireRedraw = false;
            if (consoleFrameTimer) {
                console.timeEnd(consoleFrameTimer);
                console.time(consoleFrameTimer);
            }
        }
        requestAnimationFrame(gameLoop);
    }
    requestAnimationFrame(gameLoop);
}

function addInputListeners(eventQueue, canvas, keyBind) {
    canvas.addEventListener("contextmenu", (event) => {
        event.preventDefault();
    });
    canvas.addEventListener("mousedown", (event) => {
        eventQueue.send_mouse_button(
            t, event.layerX / canvas.width, event.layerY / canvas.height, event.button, false
        );
    })
    canvas.addEventListener("mouseup", (event) => {
        eventQueue.send_mouse_button(
            t, event.layerX / canvas.width, event.layerY / canvas.height, event.button, true
        );
    })
    canvas.addEventListener("mousemove", (event) => {
        eventQueue.send_mouse_move(
            t, event.layerX / canvas.width, event.layerY / canvas.height
        );
    })
    keyBind.addEventListener("keydown", (event) => {
        if (!event.repeat) {
            eventQueue.send_key_down(event.keyCode, t);
        }
        event.preventDefault();
    });
    keyBind.addEventListener("keyup", (event) => {
        eventQueue.send_key_up(event.keyCode, t);
        event.preventDefault();
    })
}

function addWindowResizeListener(eventQueue) {
    window.addEventListener("resize", () => {
        eventQueue.send_window_resize(window.innerWidth, window.innerHeight);
    })
}
