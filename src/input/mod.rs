pub mod keyboard;
pub mod mouse;

use std::{cell::RefCell, rc::Rc};

use crate::{
    event::{EventListener, EventPipe, EventQueue, EventRouter, EventType, KeyEvent},
    num::Float,
    vector::vec2d::{Direction, Vec2d},
};

use self::keyboard::{Key, KeyMap};

pub struct InputState {
    keymap: KeyMap,
    key_pressed: [bool; 255],
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keymap: KeyMap::new(),
            key_pressed: [false; 255],
        }
    }
    pub fn with_listener(source: &mut EventRouter) -> Rc<RefCell<Self>> {
        let state = Rc::new(RefCell::new(Self::new()));
        let listener = InputStateListener(state.clone());
        source.add_listener(&[EventType::KeyUp, EventType::KeyDown], listener);
        state
    }
    pub fn register_key_up(&mut self, key: &str) {
        if let Some(x) = self.keymap.get(key) {
            self.key_pressed[x as usize] = false;
        }
    }
    pub fn register_key_down(&mut self, key: &str) {
        if let Some(x) = self.keymap.get(key) {
            self.key_pressed[x as usize] = true;
        }
    }
    pub fn keymap(&self) -> &KeyMap {
        &self.keymap
    }
    pub fn set_keymap(&mut self, keymap: KeyMap) {
        self.keymap = keymap;
    }
    pub fn is_key_pressed<T: Key>(&self, key: T) -> bool {
        let idx: u8 = key.value();
        self.key_pressed[idx as usize]
    }
}

pub struct Dpad<T> {
    input: InputStateRef,
    keys: [T; 4],
    dirs: [Direction; 4],
}

impl<T> Dpad<T>
where
    T: Key + Clone,
{
    pub fn new(input: InputStateRef, keys: [T; 4]) -> Self {
        Self {
            input,
            keys,
            dirs: [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        }
    }

    pub fn norm_v<F>(&self) -> Vec2d<F>
    where
        F: Float,
    {
        let mut v = Vec2d::zero();
        let input = self.input.borrow();
        for (k, d) in self.keys.iter().zip(self.dirs.iter()) {
            if input.is_key_pressed(k.clone()) {
                v = v + Vec2d::unit(d);
            }
        }
        v.norm()
    }
}

pub type InputStateRef = Rc<RefCell<InputState>>;

pub struct InputStateListener(Rc<RefCell<InputState>>);

impl InputStateListener {
    pub fn new(input: Rc<RefCell<InputState>>) -> Self {
        Self(input)
    }
    pub fn listen(self, source: &mut EventRouter) {
        source.add_listener(&[EventType::KeyUp, EventType::KeyDown], self);
    }
}

impl EventListener for InputStateListener {
    fn on_key_down(&mut self, event: &KeyEvent) {
        let mut input_state = self.0.borrow_mut();
        input_state.register_key_down(&event.key);
    }
    fn on_key_up(&mut self, event: &KeyEvent) {
        let mut input_state = self.0.borrow_mut();
        input_state.register_key_up(&event.key);
    }
}

pub fn bind_input(input: InputStateRef, source: &mut EventRouter) {
    let listener = InputStateListener::new(input);
    listener.listen(source);
}

pub fn bind_mouse(source: &mut EventRouter, channel: &EventQueue) {
    let pipe = EventPipe::new(channel.clone());
    source.add_listener(&[EventType::MouseClick], pipe);
}
