use wasm_retro_gamekit::game::{Game, MutStateGame};

use self::cell::ConwayCell;
use self::painter::CellAutomataPainter;
use self::world::CellAutomataWorld;

mod cell;
pub mod js;
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
