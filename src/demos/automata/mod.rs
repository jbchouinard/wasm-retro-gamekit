use crate::game::{Game, MutStateGame};

use self::{cell::ConwayCell, painter::CellAutomataPainter, world::CellAutomataWorld};

mod cell;
mod painter;
mod universe;
mod world;

pub fn conway_game_of_life(
    width: usize,
    height: usize,
    density: f32,
    generation_interval: f32,
) -> impl Game {
    let world: CellAutomataWorld<ConwayCell> =
        CellAutomataWorld::new(width, height, density, generation_interval);
    let painter: CellAutomataPainter<ConwayCell> = CellAutomataPainter::new();
    MutStateGame::new(world, painter)
}
