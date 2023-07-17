#![allow(non_snake_case)]
pub use warg::js::api::{init_once, GameHandle};
use warg::js::runner::JSGameRunner;
use wasm_bindgen::prelude::*;

use crate::conway_game_of_life;

#[wasm_bindgen]
pub fn GameOfLife(width: usize, height: usize, density: f32, interval: f32) -> GameHandle {
    let game = conway_game_of_life(width, height, density, interval);
    let runner = JSGameRunner::new(game, None);
    GameHandle::new(runner)
}
