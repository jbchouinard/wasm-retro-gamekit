use wasm_bindgen::prelude::*;

use crate::{
    automata::{cell::ConwayCell, universe::CellUniverse},
    display::Window,
    game::{FakeGame, Game},
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
pub struct WindowHandle(Window);

#[wasm_bindgen]
impl WindowHandle {
    pub fn new(game: &GameHandle) -> Self {
        Self(Window::new(game.render_width(), game.render_height()))
    }
    pub fn set_screen_size(&mut self, width: usize, height: usize) {
        self.0.rescale(width, height);
    }
    pub fn image_width(&self) -> usize {
        self.0.image_width()
    }
    pub fn image_height(&self) -> usize {
        self.0.image_height()
    }
    pub fn image_data(&self) -> *const u8 {
        self.0.image_data()
    }
    pub fn image_data_size(&self) -> usize {
        self.0.image_data_size()
    }
}

#[wasm_bindgen]
pub struct GameHandle(Box<dyn Game>);

#[wasm_bindgen]
impl GameHandle {
    pub fn render_width(&self) -> usize {
        self.0.render_width()
    }
    pub fn render_height(&self) -> usize {
        self.0.render_height()
    }
    pub fn tick(&mut self) {
        self.0.tick()
    }
    pub fn render(&mut self, window: &mut WindowHandle) {
        let mut frame = window.0.new_frame();
        self.0.render(&mut frame);
        window.0.draw_frame(&frame);
    }
}

#[wasm_bindgen]
pub fn fake_game(width: usize, height: usize) -> GameHandle {
    GameHandle(Box::new(FakeGame::new(width, height)))
}

#[wasm_bindgen]
pub fn game_of_life(width: usize, height: usize, density: f32) -> GameHandle {
    let mut universe: CellUniverse<ConwayCell> = CellUniverse::new(width, height);
    universe.randomize(density);
    GameHandle(Box::new(universe))
}
