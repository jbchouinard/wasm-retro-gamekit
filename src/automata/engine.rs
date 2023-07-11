use std::marker::PhantomData;

use crate::{
    game::{Engine, EngineResponse},
    input::{InputState, Key, KeyMap},
};

use super::{cell::Cell, universe::Universe};

pub struct CellAutomataEngine<T> {
    width: usize,
    height: usize,
    density: f32,
    last_tick: f32,
    tick_interval: f32,
    paused: bool,
    t: PhantomData<T>,
}

impl<T> CellAutomataEngine<T> {
    pub fn new(width: usize, height: usize, density: f32, tick_interval: f32) -> Self {
        Self {
            width,
            height,
            density,
            last_tick: 0.0,
            tick_interval,
            paused: false,
            t: PhantomData,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum CellAutomataKey {
    Pause = 0,
    Resume = 1,
}

impl Key for CellAutomataKey {
    fn name(&self) -> String {
        match self {
            Self::Pause => "Pause".to_string(),
            Self::Resume => "Resume".to_string(),
        }
    }

    fn value(&self) -> u8 {
        *self as u8
    }
}

impl<T> Engine<Universe<T>> for CellAutomataEngine<T>
where
    T: Cell,
{
    fn initialize(&mut self, input: &mut InputState) -> Universe<T> {
        let mut keymap = KeyMap::new();
        keymap.set_key_mapping("p", CellAutomataKey::Pause);
        keymap.set_key_mapping("r", CellAutomataKey::Resume);
        input.set_keymap(keymap);
        let mut state = Universe::new(self.width, self.height);
        state.randomize(self.density);
        state
    }

    fn tick(
        &mut self,
        state: &mut Universe<T>,
        now: f32,
        input: &mut InputState,
    ) -> EngineResponse {
        self.paused = match (
            self.paused,
            input.is_pressed(CellAutomataKey::Resume),
            input.is_pressed(CellAutomataKey::Pause),
        ) {
            (false, _, true) => true,
            (true, true, _) => false,
            (p, _, _) => p,
        };
        if self.paused {
            return EngineResponse::Empty;
        }
        if now - self.last_tick >= self.tick_interval {
            self.last_tick = now;
            match state.tick() {
                true => EngineResponse::RequestRedraw,
                false => EngineResponse::Empty,
            }
        } else {
            EngineResponse::Empty
        }
    }
}
