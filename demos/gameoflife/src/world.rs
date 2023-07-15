use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use wasm_retro_gamekit::{
    event::{EventRouter, EventType},
    game::{MutStateWorld, Response},
    input::{
        keyboard::{Key, KeyMap},
        InputState, InputStateListener,
    },
};

use super::{cell::Cell, universe::Universe};

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

pub struct CellAutomataWorld<T> {
    width: usize,
    height: usize,
    density: f32,
    last_generation_ts: f32,
    generation_interval: f32,
    paused: bool,
    input: Rc<RefCell<InputState>>,
    t: PhantomData<T>,
}

impl<T> CellAutomataWorld<T> {
    pub fn new(width: usize, height: usize, density: f32, generation_interval: f32) -> Self {
        Self {
            width,
            height,
            density,
            paused: false,
            input: Rc::new(RefCell::new(InputState::new())),
            generation_interval,
            last_generation_ts: 0.0,
            t: PhantomData,
        }
    }
    fn keymap(&self) -> KeyMap {
        let mut keymap = KeyMap::new();
        keymap.set_key_mapping("p", CellAutomataKey::Pause);
        keymap.set_key_mapping("r", CellAutomataKey::Resume);
        keymap
    }
    fn update_paused(&mut self) {
        let input = self.input.borrow();
        self.paused = match (
            self.paused,
            input.is_key_pressed(CellAutomataKey::Resume),
            input.is_key_pressed(CellAutomataKey::Pause),
        ) {
            (false, _, true) => true,
            (true, true, _) => false,
            (p, _, _) => p,
        };
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
    fn start(&mut self, _now: f32, events: &mut EventRouter) {
        events.add_listener(
            &[EventType::KeyDown, EventType::KeyUp],
            InputStateListener::new(self.input.clone()),
        );
        let mut input = self.input.borrow_mut();
        input.set_keymap(self.keymap());
    }

    fn tick(&mut self, now: f32, state: &mut Universe<T>) -> Response {
        self.update_paused();
        if self.paused {
            return Response::Empty;
        }
        if (now - self.last_generation_ts) >= self.generation_interval {
            state.tick();
            self.last_generation_ts = now;
            Response::RequestRedraw
        } else {
            Response::Empty
        }
    }
}
