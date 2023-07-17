use crate::event::Events;
use crate::graphics::Scene;

pub trait Game {
    fn start(&mut self, now: f32, events: &mut Events);
    fn tick(&mut self, now: f32) -> Response;
    fn paint(&self) -> Scene;
    fn scene_width(&self) -> usize;
    fn scene_height(&self) -> usize;
}

pub enum Response {
    Empty,
    RequestRedraw,
    Finished,
}
