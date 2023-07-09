use wasm_bindgen::prelude::*;

use crate::life::{
    cell::ConwayCell,
    universe::{CellUniverse, Universe},
};

#[wasm_bindgen]
pub fn init_once() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct GameOfLife {
    universe: Box<dyn Universe>,
}

#[wasm_bindgen]
impl GameOfLife {
    pub fn tick(&mut self) {
        self.universe.tick();
    }
    pub fn paint(&self, image_data: &mut [u8], scale: u32) {
        self.universe.draw().scaled(scale).paint(image_data);
    }
    pub fn conway(width: u32, height: u32, density: f32) -> GameOfLife {
        let mut universe: CellUniverse<ConwayCell> = CellUniverse::new(width, height);
        universe.randomize(density);
        GameOfLife {
            universe: Box::new(universe),
        }
    }
}
