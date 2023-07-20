use crate::display::Renderer;
use crate::event::Events;

pub trait Game {
    fn start(&mut self, now: f32, events: &mut Events);
    fn tick(&mut self, now: f32) -> Response;
    fn renderer(&self) -> Box<dyn Renderer>;
    fn update_resolution(&mut self, _width: usize, _height: usize) {}
    fn scene_width(&self) -> usize;
    fn scene_height(&self) -> usize;
}

pub enum Response {
    Empty,
    RequestRedraw,
    Finished,
}
