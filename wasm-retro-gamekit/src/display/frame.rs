use super::{Color, Pixels};
use crate::vector::v2::V2;

pub struct Frame(Pixels);

impl Frame {
    pub fn pixels(&self) -> &Pixels {
        &self.0
    }
    pub fn pixels_mut(&mut self) -> &mut Pixels {
        &mut self.0
    }
    pub fn width(&self) -> usize {
        self.0.width()
    }
    pub fn height(&self) -> usize {
        self.0.height()
    }
    pub fn set_pixel(&mut self, v: V2<i64>, color: Color) -> bool {
        if v.x < 0 || v.x >= self.width() as i64 || v.y < 0 || v.y >= self.height() as i64 {
            false
        } else {
            let px = self.0.get_mut(v);
            *px = color;
            true
        }
    }
}

impl Frame {
    pub fn new(pixels: Pixels) -> Self {
        Self(pixels)
    }
}
