pub mod keyboard;
pub mod mouse;

use crate::{
    event::{KeyEvent, KeyEventKind, Source},
    num::Float,
    vector::vec2d::{Direction, Vec2d},
};

use self::keyboard::{Key, KeyMap};

pub struct InputState {
    keymap: KeyMap,
    key_pressed: [bool; 255],
    events: Source<KeyEvent>,
}

impl InputState {
    pub fn new(events: Source<KeyEvent>) -> Self {
        Self {
            keymap: KeyMap::new(),
            key_pressed: [false; 255],
            events,
        }
    }
    pub fn update(&mut self) {
        while let Some(event) = self.events.recv() {
            match event.kind {
                KeyEventKind::Down => {
                    self.register_key_down(&event.key);
                }
                KeyEventKind::Up => {
                    self.register_key_up(&event.key);
                }
            }
        }
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
    keys: [T; 4],
    dirs: [Direction; 4],
}

impl<T> Dpad<T>
where
    T: Key + Clone,
{
    pub fn new(keys: [T; 4]) -> Self {
        Self {
            keys,
            dirs: [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        }
    }

    pub fn read<F>(&self, input: &InputState) -> Vec2d<F>
    where
        F: Float,
    {
        let mut v = Vec2d::zero();
        for (k, d) in self.keys.iter().zip(self.dirs.iter()) {
            if input.is_key_pressed(k.clone()) {
                v = v + Vec2d::unit(d);
            }
        }
        v.norm()
    }
}
