#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

use crate::{
    display::Window,
    event::{
        Event, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind, Sink,
        WindowResizeEvent,
    },
    game::{GameRunner, Response},
    vector::vec2d::Vec2d,
};

#[wasm_bindgen]
pub fn init_once() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console-panic")]
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
pub struct EventQueueHandle(Sink<Event>);

#[wasm_bindgen]
impl EventQueueHandle {
    pub fn send_mouse_button(&mut self, ts: f32, x: f32, y: f32, button: u8, up: bool) {
        let button = match button {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            _ => {
                return;
            }
        };
        let kind = match up {
            true => MouseEventKind::Up(button),
            false => MouseEventKind::Down(button),
        };
        self.0.send(Event::Mouse(MouseEvent {
            kind,
            pos: Vec2d::new(x, y),
            ts,
        }))
    }
    pub fn send_mouse_move(&mut self, ts: f32, x: f32, y: f32) {
        self.0.send(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Move,
            pos: Vec2d::new(x, y),
            ts,
        }))
    }
    pub fn send_key_up(&mut self, key: &str, alt: bool, ctrl: bool, shift: bool, meta: bool) {
        self.0.send(Event::Key(KeyEvent {
            kind: KeyEventKind::Up,
            key: key.to_string(),
            alt,
            ctrl,
            meta,
            shift,
        }))
    }
    pub fn send_key_down(&mut self, key: &str, alt: bool, ctrl: bool, shift: bool, meta: bool) {
        self.0.send(Event::Key(KeyEvent {
            kind: KeyEventKind::Down,
            key: key.to_string(),
            alt,
            ctrl,
            meta,
            shift,
        }))
    }
    pub fn send_window_resize(&mut self, width: u32, height: u32) {
        self.0.send(Event::WindowResize(WindowResizeEvent {
            width: width as usize,
            height: height as usize,
        }))
    }
}

#[wasm_bindgen]
pub struct GameHandle {
    game: GameRunner,
    window: Rc<RefCell<Window>>,
}

impl GameHandle {
    pub fn new(mut game: GameRunner) -> Self {
        let window = Rc::new(RefCell::new(Window::new(
            game.scene_width(),
            game.scene_height(),
            game.events().window_resize_events(),
        )));
        game.start(0.0);
        Self { game, window }
    }
}

#[wasm_bindgen]
impl GameHandle {
    pub fn window(&self) -> WindowHandle {
        WindowHandle(self.window.clone())
    }
    pub fn event_queue(&self) -> EventQueueHandle {
        EventQueueHandle(self.game.event_sink())
    }
    pub fn tick(&mut self, now: f32) -> String {
        self.window.borrow_mut().update();
        match self.game.tick(now, &mut self.window.borrow_mut()) {
            Response::Empty => "Continue",
            Response::Finished => "Finished",
            Response::RequestRedraw => "RequestRedraw",
        }
        .to_string()
    }
}
