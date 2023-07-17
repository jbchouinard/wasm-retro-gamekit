#![allow(non_snake_case)]
use wasm_bindgen::prelude::*;
pub use wasm_retro_gamekit::js::wasmbind::init_once;
use wasm_retro_gamekit::{game::GameRunner, js::wasmbind::GameHandle};

use crate::conway_game_of_life;

#[wasm_bindgen]
pub fn GameOfLife(width: usize, height: usize, density: f32, interval: f32) -> GameHandle {
    let game = conway_game_of_life(width, height, density, interval);
    let runner = GameRunner::new(game, None);
    GameHandle::new(runner)
}
