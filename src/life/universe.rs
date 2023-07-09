use super::cell::Cell;
use crate::canvas::{draw_grid, Canvas, HasColor};
use crate::grid::{Grid, Point};

pub trait Universe {
    fn tick(&mut self);
    fn randomize(&mut self, density: f32);
    fn draw(&self) -> Canvas;
}

pub struct CellUniverse<T: Cell> {
    grid: Grid<T>,
}

impl<T> CellUniverse<T>
where
    T: Cell + Default + Clone,
{
    pub fn new(width: u32, height: u32) -> Self {
        let count = (width * height) as usize;
        let mut grid: Grid<T> = vec![T::default(); count].into_iter().collect();
        grid.reshape(width, height);
        Self { grid }
    }
}

impl<T> CellUniverse<T>
where
    T: Cell,
{
    fn get(&self, p: &Point) -> &T {
        self.grid.get(p)
    }
    fn get_mut(&mut self, p: &Point) -> &mut T {
        self.grid.get_mut(p)
    }
    fn neighbors(&self, p: &Point) -> Vec<T> {
        let offsets = [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1),
        ];
        offsets
            .iter()
            .map(|x| *p + *x)
            .map(|p| self.get(&p))
            .cloned()
            .collect()
    }
}

impl<T> Universe for CellUniverse<T>
where
    T: Cell + HasColor + Clone,
{
    fn tick(&mut self) {
        for p in self.grid.iter_points() {
            let neighbors: Vec<T> = self.neighbors(&p);
            let cell = self.get_mut(&p);
            cell.tick(&neighbors);
        }
    }

    fn randomize(&mut self, density: f32) {
        for p in self.grid.iter_points() {
            let cell = self.grid.get_mut(&p);
            cell.randomize(density);
        }
    }

    fn draw(&self) -> Canvas {
        draw_grid(&self.grid)
    }
}
