mod palette;
pub mod parametric;
mod scene;
mod sprite;

pub use self::palette::*;
pub use self::scene::*;
pub use self::sprite::*;

pub trait Paint {
    fn paint(&self, palette: PaletteRef) -> Option<Sprite>;
    fn palette(&self) -> Option<PaletteRef> {
        None
    }
}
