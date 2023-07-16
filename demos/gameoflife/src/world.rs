use std::marker::PhantomData;

use wasm_retro_gamekit::{
    event::Events,
    game::{MutStateWorld, Response},
};

use super::{cell::Cell, universe::Universe};

pub struct CellAutomataWorld<T> {
    width: usize,
    height: usize,
    density: f32,
    last_generation_ts: f32,
    generation_interval: f32,
    t: PhantomData<T>,
}

impl<T> CellAutomataWorld<T> {
    pub fn new(width: usize, height: usize, density: f32, generation_interval: f32) -> Self {
        Self {
            width,
            height,
            density,
            generation_interval,
            last_generation_ts: 0.0,
            t: PhantomData,
        }
    }
}

impl<T> MutStateWorld<Universe<T>> for CellAutomataWorld<T>
where
    T: Cell,
{
    fn initial_state(&mut self) -> Universe<T> {
        let mut state = Universe::new(self.width, self.height);
        state.randomize(self.density);
        state
    }
    fn start(&mut self, _now: f32, _events: &mut Events) {}

    fn tick(&mut self, now: f32, state: &mut Universe<T>) -> Response {
        if (now - self.last_generation_ts) >= self.generation_interval {
            state.tick();
            self.last_generation_ts = now;
            Response::RequestRedraw
        } else {
            Response::Empty
        }
    }
}
