#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

use crate::{
    demos::{automata::conway_game_of_life, bouncybox::bouncy_box_game},
    display::{Window, WindowResizeListener},
    event::{Event, EventQueue, EventSource, EventType, KeyEvent, WindowResizeEvent},
    game::{GameRunner, Response},
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
pub struct WindowHandle(Rc<RefCell<Window>>);

#[wasm_bindgen]
impl WindowHandle {
    pub fn image_width(&self) -> u32 {
        self.0.borrow().image_width() as u32
    }
    pub fn image_height(&self) -> u32 {
        self.0.borrow().image_height() as u32
    }
    pub fn image_data_ptr(&self) -> *const u8 {
        self.0.borrow().image_data()
    }
    pub fn image_data_size(&self) -> usize {
        self.0.borrow().image_data_size()
    }
}

#[wasm_bindgen]
pub struct EventQueueHandle(EventQueue);

#[wasm_bindgen]
impl EventQueueHandle {
    pub fn send_key_up(
        &mut self,
        key: &str,
        alt: bool,
        ctrl: bool,
        shift: bool,
        meta: bool,
    ) -> bool {
        self.0
            .send(Event::KeyUp(KeyEvent {
                key: key.to_string(),
                alt,
                ctrl,
                meta,
                shift,
            }))
            .is_ok()
    }
    pub fn send_key_down(
        &mut self,
        key: &str,
        alt: bool,
        ctrl: bool,
        shift: bool,
        meta: bool,
    ) -> bool {
        self.0
            .send(Event::KeyDown(KeyEvent {
                key: key.to_string(),
                alt,
                ctrl,
                meta,
                shift,
            }))
            .is_ok()
    }
    pub fn send_window_resize(&mut self, width: u32, height: u32) -> bool {
        self.0
            .send(Event::WindowResize(WindowResizeEvent {
                width: width as usize,
                height: height as usize,
            }))
            .is_ok()
    }
}

#[wasm_bindgen]
pub struct GameHandle {
    game: GameRunner,
    window: Rc<RefCell<Window>>,
    event_queue: EventQueue,
}

impl GameHandle {
    fn new(mut game: GameRunner) -> Self {
        let eq = EventQueue::new(0);
        let mut source = EventSource::new(eq.clone());
        let window = Rc::new(RefCell::new(Window::new(
            game.scene_width(),
            game.scene_height(),
        )));
        source.add_listener(
            &[EventType::WindowResize],
            WindowResizeListener::new(window.clone()),
        );
        game.start(0.0, source);
        Self {
            game,
            window,
            event_queue: eq,
        }
    }
}

#[wasm_bindgen]
impl GameHandle {
    pub fn window(&self) -> WindowHandle {
        WindowHandle(self.window.clone())
    }
    pub fn event_queue(&self) -> EventQueueHandle {
        EventQueueHandle(self.event_queue.clone())
    }
    pub fn tick(&mut self, now: f32) -> String {
        self.game.poll();
        match self.game.tick(now, &mut self.window.borrow_mut()) {
            Response::Empty => "Continue",
            Response::Finished => "Finished",
            Response::RequestRedraw => "RequestRedraw",
        }
        .to_string()
    }
}

#[wasm_bindgen]
pub fn GameOfLife(width: usize, height: usize, density: f32, tick_interval: f32) -> GameHandle {
    let game = conway_game_of_life(width, height, density, tick_interval);
    let runner = GameRunner::new(game, Some(30.0));
    GameHandle::new(runner)
}

#[wasm_bindgen]
pub fn BouncyBox(width: usize, height: usize, cor: f32) -> GameHandle {
    let game = bouncy_box_game(width, height, cor);
    let runner = GameRunner::new(game, None);
    GameHandle::new(runner)
}
