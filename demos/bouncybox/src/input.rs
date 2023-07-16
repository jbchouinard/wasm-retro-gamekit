use wasm_retro_gamekit::{
    event::{KeyEvent, Source},
    input::{keyboard::KeyMap, Dpad, InputState},
};

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

pub fn dpad() -> Dpad<Keys> {
    Dpad::new([Keys::Up, Keys::Down, Keys::Left, Keys::Right])
}

pub fn inputs(events: Source<KeyEvent>) -> InputState {
    let mut input = InputState::new(events);
    input.set_keymap(keymap());
    input
}
