use crate::{
    game::{EPGame, Game},
    input::InputState,
};

use self::{cell::ConwayCell, engine::CellAutomataEngine, painter::CellAutomataPainter};

mod cell;
mod engine;
mod painter;
mod universe;

pub fn conway_game_of_life(
    input_state: &mut InputState,
    width: usize,
    height: usize,
    density: f32,
    tick_interval: f32,
) -> impl Game {
    let engine: CellAutomataEngine<ConwayCell> =
        CellAutomataEngine::new(width, height, density, tick_interval);
    let painter: CellAutomataPainter<ConwayCell> = CellAutomataPainter::new();
    EPGame::new(engine, painter, input_state)
}
