pub mod keyboard;
pub mod keycodes;
pub mod mouse;

use std::hash::Hash;

use crate::{
    num::Float,
    vector::vec2d::{Direction, Vec2d},
};

use self::keyboard::Keyboard;

pub struct Dpad<T> {
    keys: [T; 4],
    dirs: [Direction; 4],
}

impl<T> Dpad<T>
where
    T: Clone + PartialEq + Eq + Hash,
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

    pub fn read<F>(&self, input: &Keyboard<T>) -> Vec2d<F>
    where
        F: Float,
    {
        let mut v = Vec2d::zero();
        for (k, d) in self.keys.iter().zip(self.dirs.iter()) {
            if input.is_down(k) {
                v = v + Vec2d::unit(d);
            }
        }
        v.norm()
    }
}
