use super::api::GameHandle;
use super::display::JSCanvasWindow;
use crate::display::Window;
use crate::event::{Event, Events, Pump, Sink};
use crate::game::{Game, Response};

pub struct JSGameRunner {
    game: Box<dyn Game>,
    last_render_t: f32,
    min_render_t: Option<f32>,
    last_tick_t: f32,
    min_tick_t: Option<f32>,
    finished: bool,
    need_render: bool,
    events: Events,
    event_sink: Sink<Event>,
}

impl JSGameRunner {
    pub fn new<T>(game: T, fps_cap: Option<f32>, tps_cap: Option<f32>) -> Self
    where
        T: Game + 'static,
    {
        let (events, event_sink) = Events::new();
        Self {
            game: Box::new(game),
            last_render_t: 0.0,
            last_tick_t: 0.0,
            min_render_t: fps_cap.map(|x| 1000.0 / x),
            min_tick_t: tps_cap.map(|x| 1000.0 / x),
            finished: false,
            need_render: true,
            events,
            event_sink,
        }
    }

    pub fn start(&mut self, now: f32) {
        self.game.start(now, &mut self.events);
    }

    pub fn events(&mut self) -> &mut Events {
        &mut self.events
    }

    pub fn event_sink(&self) -> Sink<Event> {
        self.event_sink.clone()
    }

    pub fn tick_and_render(&mut self, now: f32, window: &mut JSCanvasWindow) -> Response {
        if self.finished {
            return Response::Finished;
        }
        if let Some(min_tick_t) = self.min_tick_t {
            if (now - self.last_tick_t) < min_tick_t {
                return Response::Empty;
            }
        }
        self.last_tick_t = now;
        self.events.pump();
        match self.game.tick(now) {
            Response::Empty => (),
            Response::RequestRedraw => {
                self.need_render = true;
            },
            Response::Finished => {
                self.finished = true;
            },
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

    fn render(&mut self, window: &mut JSCanvasWindow) {
        self.game
            .update_resolution(window.max_width(), window.max_height());
        let mut frame = window.new_frame(self.game.scene_width(), self.game.scene_height());
        let renderer = self.game.renderer();
        renderer.render(&mut frame);
        window.draw_frame(&frame);
    }

    pub fn scene_height(&self) -> usize {
        self.game.scene_height()
    }

    pub fn scene_width(&self) -> usize {
        self.game.scene_width()
    }

    pub fn handle(self) -> GameHandle {
        GameHandle::new(self)
    }
}
