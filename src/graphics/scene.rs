use std::collections::HashMap;

use super::{Layer, PColor, PaletteRef, Sprite, SpritePixelsRef, LAYERS};
use crate::display::Frame;
use crate::vector::v2::V2;

pub struct Scene {
    width: usize,
    height: usize,
    bg: Option<Background>,
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
            bg: None,
            sprites,
        }
    }

    pub fn set_background(&mut self, background: Option<Background>) {
        self.bg = background;
    }

    pub fn add_sprite(&mut self, sprite: Sprite) -> bool {
        if self.is_out_of_bounds(&sprite) {
            false
        } else {
            let layer = sprite.layer;
            self.sprites.get_mut(&layer).unwrap().push(sprite);
            true
        }
    }

    fn render_background(&self, frame: &mut Frame) {
        if let Some(background) = &self.bg {
            let palette = background.palette.colors();
            let pixels = frame.pixels();
            for v in pixels.iter_v() {
                let pcolor = background.pixels.get_pixel(v);
                match pcolor {
                    PColor::T => (),
                    _ => *pixels.get_mut(v) = palette[*pcolor as usize],
                }
            }
        }
    }

    fn is_out_of_bounds(&self, sprite: &Sprite) -> bool {
        let vtl = sprite.pos();
        let image = sprite.image();

        vtl.x > (self.width as i64)
            || (vtl.x + image.width() as i64) < 0
            || vtl.y > self.height as i64
            || (vtl.y + image.height() as i64) < 0
    }

    fn render_sprite(&self, sprite: &Sprite, frame: &mut Frame) {
        let image = sprite.image();
        let palette = sprite.palette().colors();

        let v_img_tl = sprite.pos();
        let v_img_br = v_img_tl + V2::new(image.width() as i64, image.height() as i64);

        let scn_x_min = v_img_tl.x.max(0);
        let scn_x_max = v_img_br.x.min(self.width as i64);
        let scn_y_min = v_img_tl.y.max(0);
        let scn_y_max = v_img_br.y.min(self.height as i64);

        for y in scn_y_min..scn_y_max {
            for x in scn_x_min..scn_x_max {
                let v_scn_pxl = V2::new(x, y);
                let v_img_pxl = v_scn_pxl - v_img_tl;
                let pixel = image.get_pixel(v_img_pxl);
                match pixel {
                    PColor::T => (),
                    _ => {
                        frame.set_pixel(v_scn_pxl, palette[*pixel as usize]);
                    },
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
