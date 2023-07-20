use crate::display::{Frame, Window};
use crate::event::{Source, WindowResizeEvent};
use crate::graphics::color::Rgba32;
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
    pub fn set(&mut self, v: &V2<i64>, c: Rgba32) {
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
    max_width: usize,
    max_height: usize,
    scale: usize,
    image_data: JSImageData,
    resize_events: Source<WindowResizeEvent>,
}

impl JSCanvasWindow {
    pub fn new(max_width: usize, max_height: usize, events: Source<WindowResizeEvent>) -> Self {
        Self {
            max_width,
            max_height,
            scale: 1,
            image_data: JSImageData::new(0, 0),
            resize_events: events,
        }
    }
    pub fn rescale(&mut self, frame_width: usize, frame_height: usize) {
        if (frame_width == 0) && (frame_height == 0) {
            panic!("zero sized frame!?");
        }
        self.scale = 1;
        while frame_width * (self.scale + 1) <= self.max_width
            && frame_height * (self.scale + 1) <= self.max_height
        {
            self.scale += 1;
        }
        self.image_data = JSImageData::new(frame_width * self.scale, frame_height * self.scale);
    }
    pub fn update(&mut self) {
        while let Some(e) = self.resize_events.recv() {
            self.max_width = e.width;
            self.max_height = e.height;
        }
    }
    pub fn max_width(&self) -> usize {
        self.max_width
    }
    pub fn max_height(&self) -> usize {
        self.max_height
    }
    fn frame_width(&self) -> usize {
        self.image_width() / self.scale
    }
    fn frame_height(&self) -> usize {
        self.image_height() / self.scale
    }
    pub fn image_width(&self) -> usize {
        self.image_data.width
    }
    pub fn image_height(&self) -> usize {
        self.image_data.height
    }
    pub fn image_data(&self) -> *const u8 {
        self.image_data.data()
    }
    pub fn image_data_size(&self) -> usize {
        self.image_data.data_size()
    }
}

impl Window for JSCanvasWindow {
    fn new_frame(&mut self, width: usize, height: usize) -> Frame {
        self.rescale(width, height);
        Frame::new(width, height, vec![Rgba32::default(); width * height])
    }
    fn draw_frame(&mut self, frame: &Frame) {
        if frame.w() != self.frame_width() || frame.h() != self.frame_height() {
            panic!("trying to write frame with wrong dimensions");
        }
        let fpixels = frame.pixels();
        for fy in 0..frame.h() {
            for fx in 0..frame.w() {
                let v = V2::new(fx as i64, fy as i64);
                let color = fpixels[fy * frame.w() + fx];
                let scaled_base_v = v * self.scale as i64;
                for x in 0..self.scale {
                    for y in 0..self.scale {
                        let scaled_v = scaled_base_v + V2::new(x as i64, y as i64);
                        self.image_data.set(&scaled_v, color);
                    }
                }
            }
        }
    }
}
