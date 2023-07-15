#![allow(non_snake_case)]
use wasm_bindgen::prelude::*;
pub use wasm_retro_gamekit::js::init_once;
use wasm_retro_gamekit::{game::GameRunner, js::GameHandle};

use crate::bouncy_box_game;

#[wasm_bindgen]
pub fn BouncyBox(width: usize, height: usize, cor: f32) -> GameHandle {
    let game = bouncy_box_game(width, height, cor);
    let runner = GameRunner::new(game, None);
    GameHandle::new(runner)
}
