#![allow(clippy::new_without_default)]
use std::rc::Rc;

use warg::asset::{load_asset, Asset};
use warg::display::Renderer;
use warg::event::{Events, FileReadEvent, Source};
use warg::game::{Game, Response};
use warg::graphics::color::Color;
use warg::graphics::{color, Image, Viewport};
use warg::vector::v2::V2;

pub mod js;

pub struct WrgEditor {
    file_read_events: Option<Source<FileReadEvent>>,
    image: Rc<Image<color::Rgba32>>,
    image_view: Rc<Image<color::Rgba32>>,
    viewport: Viewport,
    scale: f64,
    need_redraw: bool,
}

impl WrgEditor {
    pub fn new() -> Self {
        let image = Rc::new(Image::new(
            100,
            100,
            vec![color::Rgba32::rgba(255, 0, 0, 255); 100 * 100],
        ));
        let viewport = Viewport::new(V2::new(0, 0), 100, 100);
        let scale = 2.0;
        let image_view = Rc::new(view_image(&viewport, scale, image.as_ref()));
        Self {
            file_read_events: None,
            image,
            viewport,
            scale,
            image_view,
            need_redraw: true,
        }
    }
}

impl Game for WrgEditor {
    fn start(&mut self, _now: f32, events: &mut Events) {
        self.file_read_events = Some(events.file_read_events());
    }

    fn tick(&mut self, _now: f32) -> Response {
        if let Some(fread_events) = &self.file_read_events {
            while let Some(event) = fread_events.recv() {
                if let Ok((blob, _)) = load_asset(event.data.as_ref()) {
                    if let Asset::RgbaImage(image) = blob.into_asset() {
                        let image: Image<color::Rgba32> = image.into();
                        self.image = Rc::new(image);
                        self.need_redraw = true;
                    }
                }
            }
        }
        if self.need_redraw {
            self.image_view = Rc::new(view_image(&self.viewport, self.scale, self.image.as_ref()));
            self.need_redraw = false;
            Response::RequestRedraw
        } else {
            Response::Empty
        }
    }

    fn renderer(&self) -> Box<dyn Renderer> {
        Box::new(self.image_view.clone())
    }

    fn update_resolution(&mut self, width: usize, height: usize) {
        if width != self.viewport.width || height != self.viewport.height {
            self.viewport.width = width;
            self.viewport.height = height;
            self.image_view = Rc::new(view_image(&self.viewport, self.scale, self.image.as_ref()));
        }
    }

    fn scene_width(&self) -> usize {
        self.viewport.width
    }

    fn scene_height(&self) -> usize {
        self.viewport.height
    }
}

fn view_image<T: Color>(viewport: &Viewport, scale: f64, image: &Image<T>) -> Image<T> {
    let mut pixels_view = Vec::with_capacity(viewport.width * viewport.height);
    for y_view in 0..viewport.height {
        for x_view in 0..viewport.width {
            let x_img = (viewport.pos.x as f64 + x_view as f64 / scale).floor() as i64;
            let y_img = (viewport.pos.y as f64 + y_view as f64 / scale).floor() as i64;
            let p = if x_img >= 0 && x_img < image.w_i64() && y_img >= 0 && y_img < image.h_i64() {
                image.pixel(x_img, y_img)
            } else {
                T::default()
            };
            pixels_view.push(p);
        }
    }
    Image::new(viewport.width, viewport.height, pixels_view)
}
