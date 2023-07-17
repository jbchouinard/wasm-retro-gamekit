use wasm_retro_gamekit::input::{
    keyboard::{KeyCode, KeyMap},
    Dpad,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Keys {
    Up,
    Down,
    Left,
    Right,
}

pub fn keymap() -> KeyMap<Keys> {
    let mut km = KeyMap::new();
    km.set(KeyCode::ArrowUp, &Keys::Up);
    km.set(KeyCode::ArrowDown, &Keys::Down);
    km.set(KeyCode::ArrowLeft, &Keys::Left);
    km.set(KeyCode::ArrowRight, &Keys::Right);
    km.set(KeyCode::W, &Keys::Up);
    km.set(KeyCode::A, &Keys::Left);
    km.set(KeyCode::S, &Keys::Down);
    km.set(KeyCode::D, &Keys::Right);
    km
}

pub fn dpad() -> Dpad<Keys> {
    Dpad::new([Keys::Up, Keys::Down, Keys::Left, Keys::Right])
}
