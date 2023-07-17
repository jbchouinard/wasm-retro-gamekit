use super::Frame;

pub trait Window {
    fn new_frame(&self) -> Frame;
    fn draw_frame(&mut self, frame: &Frame);
}
