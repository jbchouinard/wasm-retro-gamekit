export function startGameFullscreen(memory, gameHandle, canvas, consoleFrameTimer) {
    const eventQueue = gameHandle.event_queue();
    // Set initial window size
    eventQueue.send_window_resize(window.innerWidth, window.innerHeight);
    addWindowResizeListener(eventQueue);
    addInputListeners(eventQueue, canvas, window);

    startGameLoop(memory, gameHandle, canvas, consoleFrameTimer)
}

function startGameLoop(memory, gameHandle, canvas, consoleFrameTimer) {
    if (consoleFrameTimer) {
        consoleFrameTimer = String(consoleFrameTimer);
        console.time(consoleFrameTimer);
    }

    let requireRedraw = true;
    let context2d = canvas.getContext("2d");

    function gameLoop() {
        let response = gameHandle.tick(window.performance.now());
        if (response == "Finished") {
            return
        } else if (response == "RequestRedraw" || requireRedraw) {
            const gameWindow = gameHandle.window();
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
    canvas.addEventListener("click", (event) => {
        eventQueue.send_click(event.layerX / canvas.width, event.layerY / canvas.height);
    });
    keyBind.addEventListener("keydown", (event) => {
        if (!event.repeat) {
            eventQueue.send_key_down(event.key, event.altKey, event.ctrlKey, event.shiftKey, event.metaKey);
        }
    });
    keyBind.addEventListener("keyup", (event) => {
        eventQueue.send_key_up(event.key, event.altKey, event.ctrlKey, event.shiftKey, event.metaKey);
    })
}

function addWindowResizeListener(eventQueue) {
    window.addEventListener("resize", () => {
        eventQueue.send_window_resize(window.innerWidth, window.innerHeight);
    })
}
