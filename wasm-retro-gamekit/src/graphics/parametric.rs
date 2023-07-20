use std::cmp::Ordering;

use crate::graphics::color::Color;
use crate::vector::v2::V2;

pub enum Aspect {
    Fill,
    Fit,
    Stretch,
}

pub struct LinearTransform {
    base_x: f64,
    base_y: f64,
    scale_x: f64,
    scale_y: f64,
}

impl LinearTransform {
    pub fn unit_rect(width: usize, height: usize, aspect: Aspect) -> Self {
        let w = width as f64;
        let h = height as f64;
        let (short, long) = match w.partial_cmp(&h) {
            Some(Ordering::Less) => (w, h),
            _ => (h, w),
        };
        let (scale_x, scale_y) = match aspect {
            Aspect::Stretch => (2.0 / w, 2.0 / h),
            Aspect::Fit => (2.0 / short, 2.0 / short),
            Aspect::Fill => (2.0 / long, 2.0 / long),
        };
        let base_x = 0.5 + (-w / 2.0);
        let base_y = 0.5 + (-h / 2.0);
        Self {
            base_x,
            base_y,
            scale_x,
            scale_y,
        }
    }

    pub fn transform(&self, v: V2<i64>) -> V2<f64> {
        let x = (self.base_x + v.x as f64) * self.scale_x;
        let y = (self.base_y + v.y as f64) * self.scale_y;
        V2::new(x, y)
    }
    pub fn untransform(&self, v: V2<f64>) -> V2<i64> {
        let x = ((v.x / self.scale_x) - self.base_x).round() as i64;
        let y = ((v.y / self.scale_y) - self.base_y).round() as i64;
        V2::new(x, y)
    }
}

pub fn draw<F, T: Color>(width: usize, height: usize, aspect: Aspect, f: F) -> Vec<T>
where
    F: Fn(V2<f64>) -> T,
{
    let transform = LinearTransform::unit_rect(width, height, aspect);
    let mut image: Vec<T> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let v = transform.transform(V2::new(x as i64, y as i64));
            image.push(f(v));
        }
    }
    image
}

pub fn rectangle<T>(fill: T, outline: T, outline_thickness: f64) -> impl Fn(V2<f64>) -> T
where
    T: Copy,
{
    move |v| {
        if (1.0 - v.x.abs()) < outline_thickness || (1.0 - v.y.abs()) < outline_thickness {
            outline
        } else {
            fill
        }
    }
}

pub fn tile<T>(white: T, black: T) -> impl Fn(V2<f64>) -> T
where
    T: Copy,
{
    move |v| match (v.x.is_sign_positive(), v.y.is_sign_positive()) {
        (true, true) | (false, false) => white,
        _ => black,
    }
}

pub fn circle<T>(
    fill: T,
    outline: T,
    transparent: T,
    outline_thickness: f64,
) -> impl Fn(V2<f64>) -> T
where
    T: Copy,
{
    move |v| {
        let m = v.mag();
        if m < 1.0 {
            if m > (1.0 - outline_thickness) {
                outline
            } else {
                fill
            }
        } else {
            transparent
        }
    }
}
