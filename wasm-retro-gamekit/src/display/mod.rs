pub use self::color::*;
pub use self::frame::*;
pub use self::window::*;
use crate::grid::Grid;

mod color;
mod frame;
mod window;

pub type Pixels = Grid<Color>;
