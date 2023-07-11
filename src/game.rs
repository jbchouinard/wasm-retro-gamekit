use crate::{display::Window, graphics::Scene, input::InputState};

pub trait Game {
    fn render_height(&self) -> usize;
    fn render_width(&self) -> usize;
    fn tick(&mut self, now: f32, input_state: &mut InputState) -> EngineResponse;
    fn render(&self, window: &mut Window);
}

pub trait Engine<T> {
    fn initialize(&mut self, input: &mut InputState) -> T;
    fn tick(&mut self, state: &mut T, now: f32, input: &mut InputState) -> EngineResponse;
}

pub trait Painter<T> {
    fn paint(&self, state: &T) -> Scene;
    fn scene_height(&self, state: &T) -> usize;
    fn scene_width(&self, state: &T) -> usize;
}

pub enum EngineResponse {
    Empty,
    RequestRedraw,
    Finished,
}

pub struct EPGame<S, E, P> {
    state: S,
    engine: E,
    painter: P,
}

impl<S, E, P> EPGame<S, E, P>
where
    E: Engine<S>,
    P: Painter<S>,
{
    pub fn new(mut engine: E, painter: P, input_state: &mut InputState) -> Self {
        let state = engine.initialize(input_state);
        Self {
            state,
            engine,
            painter,
        }
    }
}

impl<S, E, P> Game for EPGame<S, E, P>
where
    E: Engine<S>,
    P: Painter<S>,
{
    fn render_height(&self) -> usize {
        self.painter.scene_height(&self.state)
    }
    fn render_width(&self) -> usize {
        self.painter.scene_width(&self.state)
    }
    fn tick(&mut self, now: f32, input_state: &mut InputState) -> EngineResponse {
        self.engine.tick(&mut self.state, now, input_state)
    }
    fn render(&self, window: &mut Window) {
        let mut frame = window.new_frame();
        assert_eq!(frame.width(), self.render_width());
        assert_eq!(frame.height(), self.render_height());
        let scene = self.painter.paint(&self.state);
        scene.render(&mut frame);
        window.draw_frame(&frame);
    }
}
