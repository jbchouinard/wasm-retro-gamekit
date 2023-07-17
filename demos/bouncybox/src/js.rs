#![allow(non_snake_case)]
use wasm_bindgen::prelude::*;
use wasm_retro_gamekit::game::GameRunner;
pub use wasm_retro_gamekit::js::wasmbind::init_once;
use wasm_retro_gamekit::js::wasmbind::GameHandle;

use crate::bouncy_box_world;

#[wasm_bindgen]
pub fn BouncyBox(width: usize, height: usize, cor: f32) -> GameHandle {
    let game = bouncy_box_world(width, height, cor);
    let runner = GameRunner::new(game, None);
    GameHandle::new(runner)
}
