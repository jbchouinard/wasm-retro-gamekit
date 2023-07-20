use std::rc::Rc;

use crate::graphics::color::Rgba32;
use crate::graphics::Image;

pub type Frame = Image<Rgba32>;

pub trait Window {
    fn new_frame(&mut self, width: usize, height: usize) -> Frame;
    fn draw_frame(&mut self, frame: &Frame);
}

pub trait Renderer {
    fn render(&self, frame: &mut Frame);
}

impl Renderer for Rc<Image<Rgba32>> {
    fn render(&self, frame: &mut Frame) {
        assert_eq!(self.w(), frame.w());
        assert_eq!(self.h(), frame.h());
        *frame = self.as_ref().clone();
    }
}
