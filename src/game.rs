use crate::{
    display::Window,
    event::{Event, Events, Pump, Sink},
    graphics::Scene,
};

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

pub trait MutStateWorld<T> {
    fn initial_state(&mut self) -> T;
    fn start(&mut self, now: f32, events: &mut Events);
    fn tick(&mut self, now: f32, state: &mut T) -> Response;
}

pub trait Painter<T> {
    fn paint(&self, state: &T) -> Scene;
    fn scene_width(&self, state: &T) -> usize;
    fn scene_height(&self, state: &T) -> usize;
}

pub struct MutStateGame<T, W, P> {
    state: T,
    world: W,
    painter: P,
}

impl<T, W, P> MutStateGame<T, W, P>
where
    W: MutStateWorld<T>,
    P: Painter<T>,
{
    pub fn new(mut world: W, painter: P) -> Self {
        Self {
            state: world.initial_state(),
            world,
            painter,
        }
    }
}

impl<T, W, P> Game for MutStateGame<T, W, P>
where
    W: MutStateWorld<T>,
    P: Painter<T>,
{
    fn start(&mut self, now: f32, events: &mut Events) {
        self.world.start(now, events)
    }
    fn scene_height(&self) -> usize {
        self.painter.scene_height(&self.state)
    }
    fn scene_width(&self) -> usize {
        self.painter.scene_width(&self.state)
    }
    fn tick(&mut self, now: f32) -> Response {
        self.world.tick(now, &mut self.state)
    }
    fn paint(&self) -> Scene {
        self.painter.paint(&self.state)
    }
}

pub struct GameRunner {
    game: Box<dyn Game>,
    last_render_t: f32,
    min_render_t: Option<f32>,
    finished: bool,
    need_render: bool,
    events: Events,
    event_sink: Sink<Event>,
}

impl GameRunner {
    pub fn new<T>(game: T, fps_cap: Option<f32>) -> Self
    where
        T: Game + 'static,
    {
        let (events, event_sink) = Events::new();
        Self {
            game: Box::new(game),
            last_render_t: 0.0,
            min_render_t: fps_cap.map(|x| 1000.0 / x),
            finished: false,
            need_render: true,
            events,
            event_sink,
        }
    }

    pub fn start(&mut self, now: f32) {
        self.game.start(now, &mut self.events);
    }

    fn pump(&mut self) {
        self.events.pump();
    }

    pub fn events(&mut self) -> &mut Events {
        &mut self.events
    }

    pub fn event_sink(&self) -> Sink<Event> {
        self.event_sink.clone()
    }

    pub fn tick(&mut self, now: f32, window: &mut Window) -> Response {
        if self.finished {
            return Response::Finished;
        }
        self.pump();
        match self.game.tick(now) {
            Response::Empty => (),
            Response::RequestRedraw => {
                self.need_render = true;
            }
            Response::Finished => {
                self.finished = true;
            }
        }
        if self.need_render {
            if let Some(min_render_t) = self.min_render_t {
                if (now - self.last_render_t) < min_render_t {
                    return Response::Empty;
                }
            }
            self.render(window);
            self.last_render_t = now;
            Response::RequestRedraw
        } else {
            Response::Empty
        }
    }

    fn render(&self, window: &mut Window) {
        let mut frame = window.new_frame();
        assert_eq!(frame.width(), self.game.scene_width());
        assert_eq!(frame.height(), self.game.scene_height());
        let scene = self.game.paint();
        scene.render(&mut frame);
        window.draw_frame(&frame);
    }

    pub fn scene_height(&self) -> usize {
        self.game.scene_height()
    }

    pub fn scene_width(&self) -> usize {
        self.game.scene_width()
    }
}
