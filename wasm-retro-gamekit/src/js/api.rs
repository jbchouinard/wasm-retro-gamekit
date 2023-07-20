#![allow(non_snake_case)]
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use super::runner::JSGameRunner;
use crate::event::{
    Event,
    FileReadEvent,
    KeyEvent,
    KeyEventKind,
    MouseButton,
    MouseEvent,
    MouseEventKind,
    Sink,
    WindowResizeEvent,
};
use crate::game::Response;
use crate::input::keyboard::{InvalidKeyCode, KeyCode};
use crate::js::display::JSCanvasWindow;
use crate::vector::v2::V2;

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
pub struct WindowHandle(Rc<RefCell<JSCanvasWindow>>);

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
            },
        };
        let kind = match up {
            true => MouseEventKind::Up(button),
            false => MouseEventKind::Down(button),
        };
        self.0.send(Event::Mouse(MouseEvent {
            kind,
            pos: V2::new(x, y),
            ts,
        }))
    }
    pub fn send_mouse_move(&mut self, ts: f32, x: f32, y: f32) {
        self.0.send(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Move,
            pos: V2::new(x, y),
            ts,
        }))
    }
    pub fn send_key_up(&mut self, code: u8, ts: f32) {
        let keycode_res: Result<KeyCode, InvalidKeyCode> = code.try_into();
        if let Ok(keycode) = keycode_res {
            self.0.send(Event::Key(KeyEvent {
                kind: KeyEventKind::Up,
                code: keycode,
                ts,
            }))
        }
    }
    pub fn send_key_down(&mut self, code: u8, ts: f32) {
        let keycode_res: Result<KeyCode, InvalidKeyCode> = code.try_into();
        if let Ok(keycode) = keycode_res {
            self.0.send(Event::Key(KeyEvent {
                kind: KeyEventKind::Down,
                code: keycode,
                ts,
            }))
        }
    }
    pub fn send_window_resize(&mut self, width: u32, height: u32) {
        self.0.send(Event::WindowResize(WindowResizeEvent {
            width: width as usize,
            height: height as usize,
        }))
    }
    pub fn send_file_read(&mut self, name: &str, filename: &str, data: &[u8]) -> String {
        self.0.send(Event::FileRead(FileReadEvent {
            data: Rc::new(data.to_vec()),
            name: name.to_string(),
            filename: filename.to_string(),
        }));
        format!("received {} {} {}", name, filename, data.len())
    }
}

#[wasm_bindgen]
pub struct GameHandle {
    game: JSGameRunner,
    window: Rc<RefCell<JSCanvasWindow>>,
}

impl GameHandle {
    pub fn new(mut game: JSGameRunner) -> Self {
        let window = Rc::new(RefCell::new(JSCanvasWindow::new(
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
        match self
            .game
            .tick_and_render(now, &mut self.window.borrow_mut())
        {
            Response::Empty => "Continue",
            Response::Finished => "Finished",
            Response::RequestRedraw => "RequestRedraw",
        }
        .to_string()
    }
}
