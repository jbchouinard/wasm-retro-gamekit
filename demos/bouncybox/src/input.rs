use std::{cell::RefCell, rc::Rc};

use wasm_retro_gamekit::input::{keyboard::KeyMap, Dpad, InputState, InputStateRef};

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Keys {
    Up,
    Down,
    Left,
    Right,
}

impl From<Keys> for u8 {
    fn from(val: Keys) -> Self {
        val as u8
    }
}

pub fn keymap() -> KeyMap {
    let mut km = KeyMap::new();
    km.set_key_mapping("ArrowUp", Keys::Up);
    km.set_key_mapping("ArrowDown", Keys::Down);
    km.set_key_mapping("ArrowLeft", Keys::Left);
    km.set_key_mapping("ArrowRight", Keys::Right);
    km
}

pub fn inputs() -> (InputStateRef, Dpad<Keys>) {
    let mut input = InputState::new();
    input.set_keymap(keymap());
    let inputref = Rc::new(RefCell::new(input));
    let dpad = Dpad::new(
        inputref.clone(),
        [Keys::Up, Keys::Down, Keys::Left, Keys::Right],
    );
    (inputref, dpad)
}
