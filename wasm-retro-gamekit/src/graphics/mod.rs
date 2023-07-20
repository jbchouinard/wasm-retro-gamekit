pub mod color;
mod image;
pub mod parametric;
mod scene;
mod sprite;
mod viewport;

pub use self::image::*;
pub use self::scene::*;
pub use self::sprite::*;
pub use self::viewport::*;

pub trait Paint {
    fn paint(&self) -> Option<Sprite>;
}
