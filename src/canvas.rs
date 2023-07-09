use crate::grid::Point;

use super::grid::Grid;

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
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

pub struct Canvas(Grid<Color>);

impl Canvas {
    pub fn new(grid: Grid<Color>) -> Self {
        Self(grid)
    }
}

impl Canvas {
    pub fn scaled(&self, scale: u32) -> Self {
        let mut scaled_grid: Grid<Color> =
            Grid::new(self.0.width() * scale, self.0.height() * scale);

        for p in self.0.iter_points() {
            let color = self.0.get(&p);
            let scaled_base_p = p * scale as i64;
            for x in 0..scale {
                for y in 0..scale {
                    let scaled_p = scaled_base_p + Point::new(x as i64, y as i64);
                    scaled_grid.set(&scaled_p, *color);
                }
            }
        }

        Self(scaled_grid)
    }

    pub fn paint(&self, image_data: &mut [u8]) {
        let mut idx = 0;
        for p in self.0.iter_points() {
            let color = self.0.get(&p);
            image_data[idx] = color.red;
            image_data[idx + 1] = color.green;
            image_data[idx + 2] = color.blue;
            image_data[idx + 3] = color.alpha;
            idx += 4;
        }
    }
}

pub trait HasColor {
    fn has_color(&self) -> Color;
}

pub fn draw_grid<C>(grid: &Grid<C>) -> Canvas
where
    C: HasColor,
{
    let mut colors: Grid<Color> = grid.iter().map(|c| c.has_color()).collect();
    colors.reshape(grid.width(), grid.height());
    Canvas::new(colors)
}
