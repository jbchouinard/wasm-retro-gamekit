use crate::grid::{Grid, V};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }
}

pub struct Frame {
    width: usize,
    height: usize,
    pixels: Grid<Color>,
}

impl Frame {
    pub fn pixels(&mut self) -> &mut Grid<Color> {
        &mut self.pixels
    }
}

#[derive(Clone, Default)]
struct ImageData {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl ImageData {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; 4 * width * height],
        }
    }
    pub fn data_size(&self) -> usize {
        4 * self.height * self.width
    }
    pub fn set(&mut self, v: &V, c: Color) {
        let vs = *v * 4;
        let idx = self.width * vs.y as usize + vs.x as usize;
        self.data[idx] = c.red;
        self.data[idx + 1] = c.green;
        self.data[idx + 2] = c.blue;
        self.data[idx + 3] = c.alpha;
    }
    pub fn data(&self) -> *const u8 {
        self.data.as_ptr()
    }
}

pub struct Window {
    frame_width: usize,
    frame_height: usize,
    scale: usize,
    image_data: ImageData,
}

impl Window {
    pub fn new(frame_width: usize, frame_height: usize) -> Self {
        Self {
            frame_width,
            frame_height,
            scale: 1,
            image_data: ImageData::new(frame_width, frame_height),
        }
    }
    pub fn rescale(&mut self, max_width: usize, max_height: usize) {
        self.scale = 1;
        while self.frame_width * (self.scale + 1) < max_width
            && self.frame_height * (self.scale + 1) < max_height
        {
            self.scale += 1;
        }
        self.image_data = ImageData::new(self.image_width(), self.image_height());
    }
    pub fn image_width(&self) -> usize {
        self.scale * self.frame_width
    }
    pub fn image_height(&self) -> usize {
        self.scale * self.frame_height
    }
    pub fn new_frame(&self) -> Frame {
        Frame {
            width: self.frame_width,
            height: self.frame_height,
            pixels: Grid::new(self.frame_width, self.frame_height),
        }
    }
    pub fn draw_frame(&mut self, frame: &Frame) {
        if frame.width != self.frame_width || frame.height != self.frame_height {
            panic!("trying to write frame with wrong dimensions");
        }
        for p in frame.pixels.iter_points() {
            let color = frame.pixels.get(&p);
            let scaled_base_v = p * self.scale as i64;
            for x in 0..self.scale {
                for y in 0..self.scale {
                    let scaled_v = scaled_base_v + V::new(x as i64, y as i64);
                    self.image_data.set(&scaled_v, *color);
                }
            }
        }
    }
    pub fn image_data(&self) -> *const u8 {
        self.image_data.data()
    }
    pub fn image_data_size(&self) -> usize {
        self.image_data.data_size()
    }
}
