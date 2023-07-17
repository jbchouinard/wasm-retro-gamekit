use std::rc::Rc;

use crate::display::Color;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum PColor {
    #[default]
    T = 0,
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
    C9 = 9,
    C10 = 10,
    C11 = 11,
    C12 = 12,
    C13 = 13,
    C14 = 14,
    C15 = 15,
}

pub struct Palette([Color; 16]);

impl Palette {
    pub fn new(colors: &[Color]) -> Self {
        let mut palette = [Color::rgba(0, 0, 0, 0); 16];
        for (i, c) in colors.iter().take(15).enumerate() {
            palette[i + 1] = *c;
        }
        Self(palette)
    }
    pub fn colors(&self) -> [Color; 16] {
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
