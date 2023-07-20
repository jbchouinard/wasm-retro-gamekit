use bincode::{Decode, Encode};

use crate::compress::{Compression, Data};
use crate::graphics::{color, Image};

/// Random access can be very slow if the pixels are compressed.
#[derive(Encode, Decode)]
pub struct CompressedRgbaImage {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) pixels: Data<color::Rgba32>,
}

impl CompressedRgbaImage {
    pub fn new<I>(width: usize, height: usize, pixels: I, compression: Compression) -> Self
    where
        I: IntoIterator<Item = color::Rgba32>,
    {
        Self {
            width,
            height,
            pixels: Data::from_iter(pixels, compression),
        }
    }

    pub fn from_pixels(width: usize, height: usize, pixels: Data<color::Rgba32>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }
}

impl From<CompressedRgbaImage> for Image<color::Rgba32> {
    fn from(value: CompressedRgbaImage) -> Self {
        Self::new(value.width, value.height, value.pixels.into_vec())
    }
}

#[derive(Encode, Decode)]
pub struct CompressedCm4Image {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub palette: color::ColorMap4,
    pub(crate) pixels: Data<color::Cm4>,
}

impl CompressedCm4Image {
    pub fn new<I>(
        width: usize,
        height: usize,
        palette: color::ColorMap4,
        pixels: I,
        compression: Compression,
    ) -> Self
    where
        I: IntoIterator<Item = color::Cm4>,
    {
        Self {
            width,
            height,
            palette,
            pixels: Data::from_iter(pixels, compression),
        }
    }

    pub fn from_pixels(
        width: usize,
        height: usize,
        palette: color::ColorMap4,
        pixels: Data<color::Cm4>,
    ) -> Self {
        Self {
            width,
            height,
            palette,
            pixels,
        }
    }
}

impl From<CompressedCm4Image> for Image<color::Rgba32> {
    fn from(value: CompressedCm4Image) -> Self {
        let cm4_img = Image::<color::Cm4>::new(value.width, value.height, value.pixels.into_vec());
        value.palette.map_image(&cm4_img)
    }
}
