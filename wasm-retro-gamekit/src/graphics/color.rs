use bincode::{Decode, Encode};

use super::image::Image;

pub trait Color: 'static + Default + Eq + Copy + Clone + Encode + Decode {}

impl Color for Rgba32 {}

impl Color for Cm4 {}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Encode, Decode)]
pub struct Rgba32 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Rgba32 {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::rgba(red, green, blue, 255)
    }
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Encode, Decode)]
#[repr(u8)]
pub enum Cm4 {
    #[default]
    C0 = 0,
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
    C9 = 9,
    C10 = 10,
    C11 = 11,
    C12 = 12,
    C13 = 13,
    C14 = 14,
    C15 = 15,
}

#[derive(Clone, Encode, Decode)]
pub struct ColorMap4 {
    pub colors: [Rgba32; 16],
}

impl ColorMap4 {
    pub fn new(colors: &[Rgba32]) -> Self {
        let mut palette = [Rgba32::rgba(0, 0, 0, 0); 16];
        for (i, c) in colors.iter().take(16).enumerate() {
            palette[i] = *c;
        }
        Self { colors: palette }
    }

    pub fn map_color(&self, cm4_pixel: Cm4) -> Rgba32 {
        self.colors[cm4_pixel as usize]
    }

    pub fn map_image(&self, cm4_img: &Image<Cm4>) -> Image<Rgba32> {
        let mut rgba_pixels: Vec<Rgba32> = Vec::with_capacity(cm4_img.w() * cm4_img.h());
        for cm4_pixel in cm4_img.pixels() {
            rgba_pixels.push(self.map_color(*cm4_pixel))
        }
        Image::new(cm4_img.w(), cm4_img.h(), rgba_pixels)
    }
}
