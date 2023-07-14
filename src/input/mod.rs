pub mod keyboard;
pub mod mouse;

use std::{cell::RefCell, rc::Rc};

use crate::event::{EventListener, EventSource, EventType, KeyEvent};

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
    pub fn with_listener(source: &mut EventSource) -> Rc<RefCell<Self>> {
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

pub struct InputStateListener(Rc<RefCell<InputState>>);

impl InputStateListener {
    pub fn new(input: Rc<RefCell<InputState>>) -> Self {
        Self(input)
    }
    pub fn listen(self, source: &mut EventSource) {
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
