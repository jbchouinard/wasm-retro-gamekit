use wasm_bindgen::prelude::*;

use crate::{
    automata::conway_game_of_life,
    display::Window,
    game::{EngineResponse, Game},
    input::InputState,
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
    fn new(game: &GameHandle) -> Self {
        Self(Window::new(game.0.render_width(), game.0.render_height()))
    }
    pub fn set_screen_size(&mut self, width: usize, height: usize) {
        self.0.rescale(width, height);
    }
    pub fn image_width(&self) -> u32 {
        self.0.image_width() as u32
    }
    pub fn image_height(&self) -> u32 {
        self.0.image_height() as u32
    }
    pub fn image_data(&self) -> *const u8 {
        self.0.image_data()
    }
    pub fn image_data_size(&self) -> usize {
        self.0.image_data_size()
    }
}

#[wasm_bindgen]
pub struct InputHandle(InputState);

#[wasm_bindgen]
impl InputHandle {
    pub fn key_up(&mut self, key: &str) {
        self.0.register_key_up(key);
    }
    pub fn key_down(&mut self, key: &str) {
        self.0.register_key_down(key);
    }
    pub fn key_map_help(&self) -> String {
        self.0.keymap().to_string()
    }
}

#[wasm_bindgen]
pub struct GameHandle(Box<dyn Game>);

#[wasm_bindgen]
impl GameHandle {
    pub fn tick(&mut self, now: f32, input: &mut InputHandle) -> String {
        match self.0.tick(now, &mut input.0) {
            EngineResponse::Empty => "Empty".to_string(),
            EngineResponse::RequestRedraw => "RequestRedraw".to_string(),
            EngineResponse::Finished => "Finished".to_string(),
        }
    }
    pub fn render(&mut self, window: &mut WindowHandle) {
        self.0.render(&mut window.0);
    }
}

#[wasm_bindgen]
pub fn make_input() -> InputHandle {
    InputHandle(InputState::new())
}

#[wasm_bindgen]
pub fn make_game_window(game: &GameHandle) -> WindowHandle {
    WindowHandle::new(game)
}

#[wasm_bindgen]
pub fn make_game_of_life(
    input: &mut InputHandle,
    width: usize,
    height: usize,
    density: f32,
    tick_interval: f32,
) -> GameHandle {
    let game = conway_game_of_life(&mut input.0, width, height, density, tick_interval);
    GameHandle(Box::new(game))
}
