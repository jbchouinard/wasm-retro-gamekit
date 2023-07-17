use crate::display::{Color, Frame, Pixels, Window};
use crate::event::{Source, WindowResizeEvent};
use crate::vector::v2::V2;

#[derive(Clone, Default)]
struct JSImageData {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl JSImageData {
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

pub struct JSCanvasWindow {
    frame_width: usize,
    frame_height: usize,
    scale: usize,
    image_data: JSImageData,
    resize_events: Source<WindowResizeEvent>,
}

impl JSCanvasWindow {
    pub fn new(frame_width: usize, frame_height: usize, events: Source<WindowResizeEvent>) -> Self {
        Self {
            frame_width,
            frame_height,
            scale: 1,
            image_data: JSImageData::new(frame_width, frame_height),
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
        self.image_data = JSImageData::new(self.image_width(), self.image_height());
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

    pub fn image_data(&self) -> *const u8 {
        self.image_data.data()
    }
    pub fn image_data_size(&self) -> usize {
        self.image_data.data_size()
    }
}

impl Window for JSCanvasWindow {
    fn new_frame(&self) -> Frame {
        let mut pixels = Pixels::new(self.frame_width, self.frame_height);
        pixels.nowrap();
        Frame::new(pixels)
    }
    fn draw_frame(&mut self, frame: &Frame) {
        if frame.width() != self.frame_width || frame.height() != self.frame_height {
            panic!("trying to write frame with wrong dimensions");
        }
        for v in frame.pixels().iter_v() {
            let color = frame.pixels().get(v);
            let scaled_base_v = v * self.scale as i64;
            for x in 0..self.scale {
                for y in 0..self.scale {
                    let scaled_v = scaled_base_v + V2::new(x as i64, y as i64);
                    self.image_data.set(&scaled_v, *color);
                }
            }
        }
    }
}
