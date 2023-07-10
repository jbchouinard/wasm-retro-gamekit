use crate::display::{Color, Frame};

pub trait Game {
    fn tick(&mut self);
    fn render(&self, frame: &mut Frame);
    fn render_height(&self) -> usize;
    fn render_width(&self) -> usize;
}

pub struct FakeGame {
    w: usize,
    h: usize,
    counter: u64,
}

impl FakeGame {
    pub fn new(h: usize, w: usize) -> Self {
        Self { h, w, counter: 0 }
    }
}

impl Game for FakeGame {
    fn tick(&mut self) {
        self.counter = (self.counter % u64::MAX) + 1;
    }

    fn render(&self, frame: &mut Frame) {
        let pixels = frame.pixels();
        let offset = (self.counter % 2) as u8;
        for v in pixels.iter_points() {
            let on = ((offset as i64 + v.x + v.y) % 2) as u8;
            let x = 255 * on;
            pixels.set(&v, Color::rgb(x, x, x));
        }
    }

    fn render_height(&self) -> usize {
        self.h
    }

    fn render_width(&self) -> usize {
        self.w
    }
}
