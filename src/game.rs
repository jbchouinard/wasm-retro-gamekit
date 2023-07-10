use crate::display::Frame;

pub trait Game {
    fn tick(&mut self);
    fn render(&self, frame: &mut Frame);
    fn render_height(&self) -> usize;
    fn render_width(&self) -> usize;
}
