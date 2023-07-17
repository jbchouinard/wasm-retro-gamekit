pub mod keyboard;
pub mod keycodes;
pub mod mouse;

use std::hash::Hash;

use self::keyboard::Keyboard;
use crate::num::Float;
use crate::vector::v2::{Direction, V2};

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

    pub fn read<F>(&self, input: &Keyboard<T>) -> V2<F>
    where
        F: Float,
    {
        let mut v = V2::zero();
        for (k, d) in self.keys.iter().zip(self.dirs.iter()) {
            if input.is_down(k) {
                v = v + V2::unit(d);
            }
        }
        v.norm()
    }
}
