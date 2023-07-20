use std::rc::Rc;

use super::{color, parametric, Image};
use crate::vector::v2::V2;

enum SpritePixels {
    Monochrome(color::Rgba32),
    Image(Image<color::Rgba32>),
}

pub struct SpriteImage {
    width: usize,
    height: usize,
    data: SpritePixels,
}

impl SpriteImage {
    pub fn monochrome(width: usize, height: usize, color: color::Rgba32) -> SpriteImageRef {
        Rc::new(Self {
            width,
            height,
            data: SpritePixels::Monochrome(color),
        })
    }

    pub fn image(img: Image<color::Cm4>, palette: color::ColorMap4) -> SpriteImageRef {
        let rgba_img = palette.map_image(&img);
        Rc::new(Self {
            width: rgba_img.w(),
            height: rgba_img.h(),
            data: SpritePixels::Image(rgba_img),
        })
    }

    pub fn rgb_image(img: Image<color::Rgba32>) -> SpriteImageRef {
        Rc::new(Self {
            width: img.w(),
            height: img.h(),
            data: SpritePixels::Image(img),
        })
    }

    pub fn parametric<F>(
        width: usize,
        height: usize,
        palette: color::ColorMap4,
        aspect: parametric::Aspect,
        f: F,
    ) -> SpriteImageRef
    where
        F: Fn(V2<f64>) -> color::Cm4,
    {
        let cm4_pixels = parametric::draw(width, height, aspect, f);
        let cm4_img = Image::new(width, height, cm4_pixels);
        let rgba_img = palette.map_image(&cm4_img);
        Rc::new(Self {
            width,
            height,
            data: SpritePixels::Image(rgba_img),
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_pixel(&self, v: V2<i64>) -> color::Rgba32 {
        match &self.data {
            SpritePixels::Monochrome(c) => *c,
            SpritePixels::Image(image) => {
                image.pixels()[((v.y * self.width as i64) + v.x) as usize]
            },
        }
    }
}

pub type SpriteImageRef = Rc<SpriteImage>;

pub struct Sprite {
    pub(super) pos: V2<i64>,
    pub(super) layer: Layer,
    pub(super) pixels: SpriteImageRef,
}

impl Sprite {
    pub fn new(pos: V2<i64>, layer: Layer, image: SpriteImageRef) -> Self {
        Self {
            pos,
            layer,
            pixels: image,
        }
    }

    pub fn shift_pos(&mut self, v: V2<i64>) {
        self.pos = self.pos + v;
    }

    pub fn pos(&self) -> V2<i64> {
        self.pos
    }

    pub fn layer(&self) -> Layer {
        self.layer
    }

    pub fn image(&self) -> SpriteImageRef {
        self.pixels.clone()
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
