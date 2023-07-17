use crate::event::{Source, WindowResizeEvent};
use crate::grid::Grid;
use crate::vector::v2::V2;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
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

pub struct Frame {
    width: usize,
    height: usize,
    pixels: Grid<Color>,
}

impl Frame {
    pub fn pixels(&mut self) -> &mut Grid<Color> {
        &mut self.pixels
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn set_pixel(&mut self, v: V2<i64>, color: Color) -> bool {
        if v.x < 0 || v.x >= self.width as i64 || v.y < 0 || v.y >= self.height as i64 {
            false
        } else {
            let px = self.pixels.get_mut(v);
            *px = color;
            true
        }
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
    pub fn set(&mut self, v: &V2<i64>, c: Color) {
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
    resize_events: Source<WindowResizeEvent>,
}

impl Window {
    pub fn new(frame_width: usize, frame_height: usize, events: Source<WindowResizeEvent>) -> Self {
        Self {
            frame_width,
            frame_height,
            scale: 1,
            image_data: ImageData::new(frame_width, frame_height),
            resize_events: events,
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
    pub fn update(&mut self) {
        while let Some(e) = self.resize_events.recv() {
            self.rescale(e.width, e.height);
        }
    }

    pub fn image_width(&self) -> usize {
        self.scale * self.frame_width
    }
    pub fn image_height(&self) -> usize {
        self.scale * self.frame_height
    }
    pub fn new_frame(&self) -> Frame {
        let mut grid = Grid::new(self.frame_width, self.frame_height);
        grid.nowrap();
        Frame {
            width: self.frame_width,
            height: self.frame_height,
            pixels: grid,
        }
    }
    pub fn draw_frame(&mut self, frame: &Frame) {
        if frame.width != self.frame_width || frame.height != self.frame_height {
            panic!("trying to write frame with wrong dimensions");
        }
        for v in frame.pixels.iter_v() {
            let color = frame.pixels.get(v);
            let scaled_base_v = v * self.scale as i64;
            for x in 0..self.scale {
                for y in 0..self.scale {
                    let scaled_v = scaled_base_v + V2::new(x as i64, y as i64);
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
