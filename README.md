# wasm-retro-gamekit

My goal is to build from scratch a toolkit for making Retro games running in the browser
with Rust and WebAssembly.

This is a learning project, if you want to build a real game you should be using
Unity or whatever.


## Demos
- [Bouncy Boxes](https://jbchouinard.github.io/wasm-retro-gamekit/bouncybox/) ([source](demos/bouncybox))
- [Conway's Game of Life](https://jbchouinard.github.io/wasm-retro-gamekit/gameoflife/) ([source](demos/gameoflife))


## Requirements
- [Rust](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js and npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)


## Usage: Making a Game

See [Demos](#demos) for examples of small "games" built with `wasm-retro-gamekit`.

### Tutorial

(work in progress)

```sh
cargo new my-game --lib
```

Cargo.toml
```toml
[dependencies]
wasm-retro-gamekit = { git = "https://github.com/jbchouinard/wasm-retro-gamekit.git" }
```

A game consists of a Rust struct which implements the `wasm_retro_gamekit::game::Game`
trait.

The Rust library should expose a function which returns a `wasm_retro_gamekit::js::GameHandle`
struct.


## Roadmap

Current features
- 2D sprites
- 2D viewport
- 2D rectangular hitbox collision physics
- Keyboard input
- Mouse click input
- Canvas rendering with Context2D

Future features?
- Input
    - Mouse position input
    - Game controller input
    - Touch input
- Audio
    - Music
    - Sounds
- Assets
    - `fetch` based asset loading
    - Asset registries, loading/unloading, levels
    - In-browser asset editors
- Graphics
    - Sprite animations
    - Canvas rendering with WebGL
- UIs
    - Fonts and text rendering
    - Layered windows
    - Input events routing
- Physics
    - Traits and reactor for object interactions
    - Fields (e.g. gravity)
    - Friction
    - Better collisions
- Developer experience
    - Replace JS glue code with `web-sys`
    - Logging


## License

Licensed under the [Apache License, Version 2.0](LICENSE).


## Copyright

See the copyright [NOTICE](NOTICE).

---

Built with [Rust and WebAssembly](https://rustwasm.github.io/), using the [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template) template.
