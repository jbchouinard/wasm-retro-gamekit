use std::{collections::HashMap, rc::Rc};

use crate::{
    display::{Color, Frame},
    grid::{Grid, Vector},
};

#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
pub enum PColor {
    #[default]
    C1 = 0,
    C2 = 1,
    C3 = 2,
    C4 = 3,
    C5 = 4,
    C6 = 5,
    C7 = 6,
    C8 = 7,
    T = 255,
}

pub struct Palette([Color; 8]);

impl Palette {
    pub fn new(colors: [Color; 8]) -> Self {
        Self(colors)
    }
    pub fn colors(&self) -> [Color; 8] {
        self.0
    }
    pub fn color(&self, pc: PColor) -> Color {
        match pc {
            PColor::T => Color::rgba(0, 0, 0, 0),
            _ => self.0[pc as usize],
        }
    }
}

pub type PaletteRef = Rc<Palette>;

#[derive(Clone)]
pub struct SpriteImage {
    pixels: Grid<PColor>,
}

impl SpriteImage {
    pub fn new(pixels: Vec<PColor>, width: usize, height: usize) -> Self {
        assert_eq!(pixels.len(), width * height, "wrong SpriteImage dimensions");
        let mut grid: Grid<PColor> = pixels.into_iter().collect();
        grid.reshape(width, height);
        Self { pixels: grid }
    }

    pub fn width(&self) -> usize {
        self.pixels.width()
    }

    pub fn height(&self) -> usize {
        self.pixels.height()
    }

    pub fn get_pixel(&self, v: Vector) -> PColor {
        *self.pixels.get(v)
    }
}

pub type SpriteImageRef = Rc<SpriteImage>;

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

static LAYERS: [Layer; 8] = [
    Layer::L0,
    Layer::L1,
    Layer::L2,
    Layer::L3,
    Layer::L4,
    Layer::L5,
    Layer::L6,
    Layer::L7,
];

pub struct Sprite {
    pos: Vector,
    layer: Layer,
    image: SpriteImageRef,
    palette: PaletteRef,
}

impl Sprite {
    pub fn new(pos: Vector, layer: Layer, image: SpriteImageRef, palette: PaletteRef) -> Self {
        Self {
            pos,
            layer,
            image,
            palette,
        }
    }

    pub fn pos(&self) -> Vector {
        self.pos
    }

    pub fn layer(&self) -> Layer {
        self.layer
    }

    pub fn image(&self) -> SpriteImageRef {
        self.image.clone()
    }

    pub fn palette(&self) -> PaletteRef {
        self.palette.clone()
    }
}

pub struct Scene {
    width: usize,
    height: usize,
    bg_color: Color,
    sprites: HashMap<Layer, Vec<Sprite>>,
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sprites = HashMap::with_capacity(8);
        for layer in LAYERS.iter() {
            sprites.insert(*layer, vec![]);
        }
        Self {
            width,
            height,
            bg_color: Color::default(),
            sprites,
        }
    }

    pub fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        let layer = sprite.layer;
        self.sprites.get_mut(&layer).unwrap().push(sprite);
    }

    fn render_background(&self, frame: &mut Frame) {
        let pixels = frame.pixels().mut_cells();
        for p in pixels.iter_mut() {
            *p = self.bg_color;
        }
    }

    fn render_sprite(&self, sprite: &Sprite, frame: &mut Frame) {
        let vtl = sprite.pos();
        let image = sprite.image();
        let palette = sprite.palette().colors();

        if vtl.x > (self.width as i64)
            || (vtl.x + image.width() as i64) < 0
            || vtl.y > self.height as i64
            || (vtl.y + image.height() as i64) < 0
        {
            return;
        }

        for y in 0..image.height() {
            for x in 0..image.width() {
                let vs = Vector::new(x as i64, y as i64);
                let vf = vtl + vs;
                let pixel = image.get_pixel(vs);
                match pixel {
                    PColor::T => (),
                    _ => {
                        frame.set_pixel(vf, palette[pixel as usize]);
                    }
                }
            }
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let len = frame.pixels().mut_cells().len();
        assert_eq!(self.width, frame.width());
        assert_eq!(self.height, frame.height());
        self.render_background(frame);
        for layer in LAYERS.iter() {
            for sprite in self.sprites.get(layer).unwrap() {
                self.render_sprite(sprite, frame);
            }
        }
        assert_eq!(len, frame.pixels().mut_cells().len());
    }
}
