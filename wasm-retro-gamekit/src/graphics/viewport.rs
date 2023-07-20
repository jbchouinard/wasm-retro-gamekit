use super::Sprite;
use crate::num::Float;
use crate::vector::v2::V2;

pub struct Viewport {
    pub pos: V2<i64>,
    pub width: usize,
    pub height: usize,
}

impl Viewport {
    pub fn new(pos: V2<i64>, width: usize, height: usize) -> Self {
        Self { pos, width, height }
    }

    pub fn relative_pos<T>(&self, pos: V2<T>) -> V2<i64>
    where
        T: Float,
    {
        let relx: i64 = (T::from_usize(self.width).unwrap() * pos.x)
            .round()
            .to_i64()
            .unwrap();
        let rely: i64 = (T::from_usize(self.height).unwrap() * pos.y)
            .round()
            .to_i64()
            .unwrap();
        self.pos + V2::new(relx, rely)
    }

    pub fn overlaps(&self, sprite: &Sprite) -> bool {
        let image = sprite.image();
        let stl = sprite.pos() - self.pos;
        let sw = image.width() as i64;
        let sh = image.height() as i64;
        let sx0 = stl.x;
        let sx1 = stl.x + sw;
        let sy0 = stl.y;
        let sy1 = stl.y + sh;

        !(sx0 >= self.width as i64 || sx1 <= 0 || sy0 >= self.height as i64 || sy1 <= 0)
    }
}
