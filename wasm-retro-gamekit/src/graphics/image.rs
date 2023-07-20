use super::color::Color;
use crate::vector::v2::V2;

#[derive(Clone)]
pub struct Image<T: Color> {
    w: usize,
    w_i64: i64,
    h: usize,
    h_i64: i64,
    pixels: Vec<T>,
}

impl<T: Color> Image<T> {
    pub fn new(width: usize, height: usize, pixels: Vec<T>) -> Self {
        assert_eq!(pixels.len(), width * height);
        Self {
            w: width,
            w_i64: width as i64,
            h: height,
            h_i64: height as i64,
            pixels,
        }
    }

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn w_i64(&self) -> i64 {
        self.w_i64
    }

    pub fn h(&self) -> usize {
        self.h
    }

    pub fn h_i64(&self) -> i64 {
        self.h_i64
    }

    pub fn pixels(&self) -> &[T] {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut [T] {
        &mut self.pixels
    }

    fn idx(&self, x: i64, y: i64) -> usize {
        ((y * self.w_i64) + x) as usize
    }

    pub fn pixel(&self, x: i64, y: i64) -> T {
        self.pixels[self.idx(x, y)]
    }

    pub fn pixel_v(&self, v: V2<i64>) -> T {
        self.pixels[self.idx(v.x, v.y)]
    }

    pub fn pixel_mut(&mut self, x: i64, y: i64) -> &mut T {
        let idx = self.idx(x, y);
        &mut self.pixels[idx]
    }

    pub fn pixel_mut_v(&mut self, v: V2<i64>) -> &mut T {
        let idx = self.idx(v.x, v.y);
        &mut self.pixels[idx]
    }

    pub fn cropped(&self, width: usize, height: usize) -> Image<T> {
        let mut cropped_pixels = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let pixel = if x >= self.w || y >= self.h {
                    T::default()
                } else {
                    self.pixel(x as i64, y as i64)
                };
                cropped_pixels.push(pixel);
            }
        }
        Image::new(width, height, cropped_pixels)
    }
}
