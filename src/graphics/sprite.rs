use std::rc::Rc;

use crate::{
    grid::{Grid, Vector},
    vector::vec2d::Vec2d,
};

use super::{parametric, PColor, PaletteRef};

#[derive(Clone)]
pub enum SpritePixels {
    Uniform((usize, usize, PColor)),
    Image(Grid<PColor>),
}

impl SpritePixels {
    pub fn uniform(width: usize, height: usize, color: PColor) -> SpritePixelsRef {
        Rc::new(Self::Uniform((width, height, color)))
    }

    pub fn image(width: usize, height: usize, data: Vec<PColor>) -> SpritePixelsRef {
        assert_eq!(data.len(), width * height, "image data has wrong size");
        let mut grid: Grid<PColor> = data.into_iter().collect();
        grid.reshape(width, height);
        Rc::new(Self::Image(grid))
    }

    pub fn parametric<F>(
        width: usize,
        height: usize,
        aspect: parametric::Aspect,
        f: F,
    ) -> SpritePixelsRef
    where
        F: Fn(Vec2d<f64>) -> PColor,
    {
        let pixels = parametric::draw(width, height, aspect, f);
        Self::image(width, height, pixels)
    }

    pub fn width(&self) -> usize {
        match self {
            Self::Uniform((w, _, _)) => *w,
            Self::Image(grid) => grid.width(),
        }
    }

    pub fn height(&self) -> usize {
        match self {
            Self::Uniform((_, h, _)) => *h,
            Self::Image(grid) => grid.height(),
        }
    }

    pub fn get_pixel(&self, v: Vector) -> &PColor {
        match self {
            Self::Uniform((_, _, c)) => c,
            Self::Image(grid) => grid.get(v),
        }
    }
}

pub type SpritePixelsRef = Rc<SpritePixels>;

#[derive(Clone)]
pub struct Background {
    pub(super) pixels: SpritePixelsRef,
    pub(super) palette: PaletteRef,
}

impl Background {
    pub fn new(image: SpritePixelsRef, palette: PaletteRef) -> Self {
        Self {
            pixels: image,
            palette,
        }
    }
}

pub struct Sprite {
    pub(super) pos: Vector,
    pub(super) layer: Layer,
    pub(super) pixels: SpritePixelsRef,
    pub(super) palette: PaletteRef,
}

impl Sprite {
    pub fn new(pos: Vector, layer: Layer, image: SpritePixelsRef, palette: PaletteRef) -> Self {
        Self {
            pos,
            layer,
            pixels: image,
            palette,
        }
    }

    pub fn shift_pos(&mut self, v: Vector) {
        self.pos = self.pos + v;
    }

    pub fn pos(&self) -> Vector {
        self.pos
    }

    pub fn layer(&self) -> Layer {
        self.layer
    }

    pub fn image(&self) -> SpritePixelsRef {
        self.pixels.clone()
    }

    pub fn palette(&self) -> PaletteRef {
        self.palette.clone()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Layer {
    L0 = 0,
    L1 = 1,
    L2 = 2,
    L3 = 3,
    L4 = 4,
    L5 = 5,
    L6 = 6,
    L7 = 7,
}

pub(super) static LAYERS: [Layer; 8] = [
    Layer::L0,
    Layer::L1,
    Layer::L2,
    Layer::L3,
    Layer::L4,
    Layer::L5,
    Layer::L6,
    Layer::L7,
];
