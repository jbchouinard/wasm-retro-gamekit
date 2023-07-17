use warg::game::Game;

use self::cell::ConwayCell;
use self::game::CellAutomataWorld;

mod cell;
mod game;
pub mod js;
mod universe;

pub fn conway_game_of_life(
    width: usize,
    height: usize,
    density: f32,
    generation_interval: f32,
) -> impl Game {
    let game: CellAutomataWorld<ConwayCell> =
        CellAutomataWorld::new(width, height, density, generation_interval);
    game
}
