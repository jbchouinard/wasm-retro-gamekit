#![allow(non_snake_case)]
pub use warg::js::api::{init_once, GameHandle};
use warg::js::runner::JSGameRunner;
use wasm_bindgen::prelude::*;

use crate::bouncy_box_world;

#[wasm_bindgen]
pub fn BouncyBox(width: usize, height: usize, cor: f32) -> GameHandle {
    let game = bouncy_box_world(width, height, cor);
    let runner = JSGameRunner::new(game, None);
    GameHandle::new(runner)
}
