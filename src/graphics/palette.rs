use std::rc::Rc;

use crate::display::Color;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum PColor {
    #[default]
    C1 = 0,
    C2 = 1,
    C3 = 2,
    C4 = 3,
    C5 = 4,
    C6 = 5,
    C7 = 6,
    C8 = 7,
    T = 255,
}

pub struct Palette([Color; 8]);

impl Palette {
    pub fn new(colors: [Color; 8]) -> Self {
        Self(colors)
    }
    pub fn colors(&self) -> [Color; 8] {
        self.0
    }
    pub fn color(&self, pc: PColor) -> Color {
        match pc {
            PColor::T => Color::rgba(0, 0, 0, 0),
            _ => self.0[pc as usize],
        }
    }
}

pub type PaletteRef = Rc<Palette>;
