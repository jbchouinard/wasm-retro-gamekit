"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunkwasm_retro_gamekit_demo_bouncybox"] = self["webpackChunkwasm_retro_gamekit_demo_bouncybox"] || []).push([["index_js"],{

/***/ "../pkg/wasm_retro_gamekit_bouncybox.js":
/*!**********************************************!*\
  !*** ../pkg/wasm_retro_gamekit_bouncybox.js ***!
  \**********************************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   BouncyBox: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.BouncyBox),\n/* harmony export */   EventQueueHandle: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.EventQueueHandle),\n/* harmony export */   GameHandle: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.GameHandle),\n/* harmony export */   WindowHandle: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.WindowHandle),\n/* harmony export */   __wbg_error_f851667af71bcfc6: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_error_f851667af71bcfc6),\n/* harmony export */   __wbg_new_abda76e883ba8a5f: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_abda76e883ba8a5f),\n/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   __wbg_stack_658279fe44541cf6: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_stack_658279fe44541cf6),\n/* harmony export */   __wbindgen_object_drop_ref: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_drop_ref),\n/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw),\n/* harmony export */   init_once: () => (/* reexport safe */ _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.init_once)\n/* harmony export */ });\n/* harmony import */ var _wasm_retro_gamekit_bouncybox_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./wasm_retro_gamekit_bouncybox_bg.wasm */ \"../pkg/wasm_retro_gamekit_bouncybox_bg.wasm\");\n/* harmony import */ var _wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_retro_gamekit_bouncybox_bg.js */ \"../pkg/wasm_retro_gamekit_bouncybox_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_wasm_retro_gamekit_bouncybox_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_wasm_retro_gamekit_bouncybox_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n(0,_wasm_retro_gamekit_bouncybox_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_wasm_retro_gamekit_bouncybox_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://wasm-retro-gamekit-demo-bouncybox/../pkg/wasm_retro_gamekit_bouncybox.js?");

/***/ }),

/***/ "../pkg/wasm_retro_gamekit_bouncybox_bg.js":
/*!*************************************************!*\
  !*** ../pkg/wasm_retro_gamekit_bouncybox_bg.js ***!
  \*************************************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   BouncyBox: () => (/* binding */ BouncyBox),\n/* harmony export */   EventQueueHandle: () => (/* binding */ EventQueueHandle),\n/* harmony export */   GameHandle: () => (/* binding */ GameHandle),\n/* harmony export */   WindowHandle: () => (/* binding */ WindowHandle),\n/* harmony export */   __wbg_error_f851667af71bcfc6: () => (/* binding */ __wbg_error_f851667af71bcfc6),\n/* harmony export */   __wbg_new_abda76e883ba8a5f: () => (/* binding */ __wbg_new_abda76e883ba8a5f),\n/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   __wbg_stack_658279fe44541cf6: () => (/* binding */ __wbg_stack_658279fe44541cf6),\n/* harmony export */   __wbindgen_object_drop_ref: () => (/* binding */ __wbindgen_object_drop_ref),\n/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw),\n/* harmony export */   init_once: () => (/* binding */ init_once)\n/* harmony export */ });\n/* module decorator */ module = __webpack_require__.hmd(module);\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nconst heap = new Array(128).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 132) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {number} width\n* @param {number} height\n* @param {number} cor\n* @returns {GameHandle}\n*/\nfunction BouncyBox(width, height, cor) {\n    const ret = wasm.BouncyBox(width, height, cor);\n    return GameHandle.__wrap(ret);\n}\n\n/**\n*/\nfunction init_once() {\n    wasm.init_once();\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length, 1) >>> 0;\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len, 1) >>> 0;\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0 = null;\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);\n    }\n    return cachedInt32Memory0;\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n/**\n*/\nclass EventQueueHandle {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(EventQueueHandle.prototype);\n        obj.__wbg_ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_eventqueuehandle_free(ptr);\n    }\n    /**\n    * @param {number} x\n    * @param {number} y\n    * @returns {boolean}\n    */\n    send_click(x, y) {\n        const ret = wasm.eventqueuehandle_send_click(this.__wbg_ptr, x, y);\n        return ret !== 0;\n    }\n    /**\n    * @param {string} key\n    * @param {boolean} alt\n    * @param {boolean} ctrl\n    * @param {boolean} shift\n    * @param {boolean} meta\n    * @returns {boolean}\n    */\n    send_key_up(key, alt, ctrl, shift, meta) {\n        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len0 = WASM_VECTOR_LEN;\n        const ret = wasm.eventqueuehandle_send_key_up(this.__wbg_ptr, ptr0, len0, alt, ctrl, shift, meta);\n        return ret !== 0;\n    }\n    /**\n    * @param {string} key\n    * @param {boolean} alt\n    * @param {boolean} ctrl\n    * @param {boolean} shift\n    * @param {boolean} meta\n    * @returns {boolean}\n    */\n    send_key_down(key, alt, ctrl, shift, meta) {\n        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len0 = WASM_VECTOR_LEN;\n        const ret = wasm.eventqueuehandle_send_key_down(this.__wbg_ptr, ptr0, len0, alt, ctrl, shift, meta);\n        return ret !== 0;\n    }\n    /**\n    * @param {number} width\n    * @param {number} height\n    * @returns {boolean}\n    */\n    send_window_resize(width, height) {\n        const ret = wasm.eventqueuehandle_send_window_resize(this.__wbg_ptr, width, height);\n        return ret !== 0;\n    }\n}\n/**\n*/\nclass GameHandle {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(GameHandle.prototype);\n        obj.__wbg_ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_gamehandle_free(ptr);\n    }\n    /**\n    * @returns {WindowHandle}\n    */\n    window() {\n        const ret = wasm.gamehandle_window(this.__wbg_ptr);\n        return WindowHandle.__wrap(ret);\n    }\n    /**\n    * @returns {EventQueueHandle}\n    */\n    event_queue() {\n        const ret = wasm.gamehandle_event_queue(this.__wbg_ptr);\n        return EventQueueHandle.__wrap(ret);\n    }\n    /**\n    * @param {number} now\n    * @returns {string}\n    */\n    tick(now) {\n        let deferred1_0;\n        let deferred1_1;\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.gamehandle_tick(retptr, this.__wbg_ptr, now);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            deferred1_0 = r0;\n            deferred1_1 = r1;\n            return getStringFromWasm0(r0, r1);\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);\n        }\n    }\n}\n/**\n*/\nclass WindowHandle {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(WindowHandle.prototype);\n        obj.__wbg_ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_windowhandle_free(ptr);\n    }\n    /**\n    * @returns {number}\n    */\n    image_width() {\n        const ret = wasm.windowhandle_image_width(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number}\n    */\n    image_height() {\n        const ret = wasm.windowhandle_image_height(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number}\n    */\n    image_data_ptr() {\n        const ret = wasm.windowhandle_image_data_ptr(this.__wbg_ptr);\n        return ret;\n    }\n    /**\n    * @returns {number}\n    */\n    image_data_size() {\n        const ret = wasm.windowhandle_image_data_size(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n}\n\nfunction __wbg_new_abda76e883ba8a5f() {\n    const ret = new Error();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_stack_658279fe44541cf6(arg0, arg1) {\n    const ret = getObject(arg1).stack;\n    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n    const len1 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len1;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr1;\n};\n\nfunction __wbg_error_f851667af71bcfc6(arg0, arg1) {\n    let deferred0_0;\n    let deferred0_1;\n    try {\n        deferred0_0 = arg0;\n        deferred0_1 = arg1;\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);\n    }\n};\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n\n//# sourceURL=webpack://wasm-retro-gamekit-demo-bouncybox/../pkg/wasm_retro_gamekit_bouncybox_bg.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_retro_gamekit_bouncybox_wasm_retro_gamekit_bouncybox_bg__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! wasm_retro_gamekit_bouncybox/wasm_retro_gamekit_bouncybox_bg */ \"../pkg/wasm_retro_gamekit_bouncybox_bg.js\");\n/* harmony import */ var wasm_retro_gamekit_bouncybox__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm_retro_gamekit_bouncybox */ \"../pkg/wasm_retro_gamekit_bouncybox.js\");\n/* harmony import */ var retro_gamekit_bootstrap__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! retro-gamekit-bootstrap */ \"./node_modules/retro-gamekit-bootstrap/retro-gamekit-bootstrap.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([wasm_retro_gamekit_bouncybox__WEBPACK_IMPORTED_MODULE_0__]);\nwasm_retro_gamekit_bouncybox__WEBPACK_IMPORTED_MODULE_0__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n\n\n(0,wasm_retro_gamekit_bouncybox__WEBPACK_IMPORTED_MODULE_0__.init_once)();\n\n(0,retro_gamekit_bootstrap__WEBPACK_IMPORTED_MODULE_1__.startGameFullscreen)(\n    wasm_retro_gamekit_bouncybox_wasm_retro_gamekit_bouncybox_bg__WEBPACK_IMPORTED_MODULE_2__.memory,\n    (0,wasm_retro_gamekit_bouncybox__WEBPACK_IMPORTED_MODULE_0__.BouncyBox)(window.innerWidth, window.innerHeight, 0.8),\n    document.getElementById(\"game-canvas\"),\n);\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://wasm-retro-gamekit-demo-bouncybox/./index.js?");

/***/ }),

/***/ "./node_modules/retro-gamekit-bootstrap/retro-gamekit-bootstrap.js":
/*!*************************************************************************!*\
  !*** ./node_modules/retro-gamekit-bootstrap/retro-gamekit-bootstrap.js ***!
  \*************************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   startGameFullscreen: () => (/* binding */ startGameFullscreen)\n/* harmony export */ });\nfunction startGameFullscreen(memory, gameHandle, canvas, consoleFrameTimer) {\n    const eventQueue = gameHandle.event_queue();\n    // Set initial window size\n    eventQueue.send_window_resize(window.innerWidth, window.innerHeight);\n    addWindowResizeListener(eventQueue);\n    addInputListeners(eventQueue, canvas, window);\n\n    startGameLoop(memory, gameHandle, canvas, consoleFrameTimer)\n}\n\nfunction startGameLoop(memory, gameHandle, canvas, consoleFrameTimer) {\n    if (consoleFrameTimer) {\n        consoleFrameTimer = String(consoleFrameTimer);\n        console.time(consoleFrameTimer);\n    }\n\n    let requireRedraw = true;\n    let context2d = canvas.getContext(\"2d\");\n\n    function gameLoop() {\n        let response = gameHandle.tick(window.performance.now());\n        if (response == \"Finished\") {\n            return\n        } else if (response == \"RequestRedraw\" || requireRedraw) {\n            const gameWindow = gameHandle.window();\n            let width = gameWindow.image_width();\n            let height = gameWindow.image_height();\n            canvas.width = width;\n            canvas.height = height;\n            const imageDataArray = new Uint8ClampedArray(\n                memory.buffer,\n                gameWindow.image_data_ptr(),\n                gameWindow.image_data_size(),\n            );\n            const imageData = new ImageData(imageDataArray, width, height);\n            context2d.putImageData(imageData, 0, 0);\n            requireRedraw = false;\n            if (consoleFrameTimer) {\n                console.timeEnd(consoleFrameTimer);\n                console.time(consoleFrameTimer);\n            }\n        }\n        requestAnimationFrame(gameLoop);\n    }\n    requestAnimationFrame(gameLoop);\n}\n\nfunction addInputListeners(eventQueue, canvas, keyBind) {\n    canvas.addEventListener(\"click\", (event) => {\n        eventQueue.send_click(event.layerX / canvas.width, event.layerY / canvas.height);\n    });\n    keyBind.addEventListener(\"keydown\", (event) => {\n        if (!event.repeat) {\n            eventQueue.send_key_down(event.key, event.altKey, event.ctrlKey, event.shiftKey, event.metaKey);\n        }\n    });\n    keyBind.addEventListener(\"keyup\", (event) => {\n        eventQueue.send_key_up(event.key, event.altKey, event.ctrlKey, event.shiftKey, event.metaKey);\n    })\n}\n\nfunction addWindowResizeListener(eventQueue) {\n    window.addEventListener(\"resize\", () => {\n        eventQueue.send_window_resize(window.innerWidth, window.innerHeight);\n    })\n}\n\n\n//# sourceURL=webpack://wasm-retro-gamekit-demo-bouncybox/./node_modules/retro-gamekit-bootstrap/retro-gamekit-bootstrap.js?");

/***/ }),

/***/ "../pkg/wasm_retro_gamekit_bouncybox_bg.wasm":
/*!***************************************************!*\
  !*** ../pkg/wasm_retro_gamekit_bouncybox_bg.wasm ***!
  \***************************************************/
/***/ ((module, exports, __webpack_require__) => {

eval("/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./wasm_retro_gamekit_bouncybox_bg.js */ \"../pkg/wasm_retro_gamekit_bouncybox_bg.js\");\nmodule.exports = __webpack_require__.v(exports, module.id, \"a97bfcd9e07c824bc027\", {\n\t\"./wasm_retro_gamekit_bouncybox_bg.js\": {\n\t\t\"__wbg_new_abda76e883ba8a5f\": WEBPACK_IMPORTED_MODULE_0.__wbg_new_abda76e883ba8a5f,\n\t\t\"__wbg_stack_658279fe44541cf6\": WEBPACK_IMPORTED_MODULE_0.__wbg_stack_658279fe44541cf6,\n\t\t\"__wbg_error_f851667af71bcfc6\": WEBPACK_IMPORTED_MODULE_0.__wbg_error_f851667af71bcfc6,\n\t\t\"__wbindgen_object_drop_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref,\n\t\t\"__wbindgen_throw\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw\n\t}\n});\n\n//# sourceURL=webpack://wasm-retro-gamekit-demo-bouncybox/../pkg/wasm_retro_gamekit_bouncybox_bg.wasm?");

/***/ })

}]);