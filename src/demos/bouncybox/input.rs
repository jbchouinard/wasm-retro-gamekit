use crate::input::keyboard::KeyMap;

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
