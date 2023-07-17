use bincode::{Decode, Encode};

use crate::compress::{Compression, Data};
pub use crate::display::Color;

const COMPRESSION: Compression = Compression::Rle;

#[derive(Encode, Decode)]
pub struct ImageAsset {
    width: usize,
    height: usize,
    pixels: Data<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize, pixels: Data<Color>) -> Self {
        assert_eq!(pixels.len(), width * height);
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &Data<Color> {
        &self.pixels
    }

    pub fn compress(&mut self) {
        self.pixels.compress(COMPRESSION);
    }

    pub fn decompress(&mut self) {
        self.pixels.decompress();
    }
}
