use super::cell::Cell;
use crate::{
    display::Frame,
    game::Game,
    grid::{Grid, V},
};

pub struct CellUniverse<T: Cell> {
    grid: Grid<T>,
}

impl<T> CellUniverse<T>
where
    T: Cell + Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        let count = width * height;
        let mut grid: Grid<T> = vec![T::default(); count].into_iter().collect();
        grid.reshape(width, height);
        Self { grid }
    }
}

impl<T> CellUniverse<T>
where
    T: Cell,
{
    fn get(&self, p: &V) -> &T {
        self.grid.get(p)
    }
    fn get_mut(&mut self, p: &V) -> &mut T {
        self.grid.get_mut(p)
    }
    pub fn randomize(&mut self, density: f32) {
        for p in self.grid.iter_points() {
            let cell = self.grid.get_mut(&p);
            cell.randomize(density);
        }
    }
}

pub fn draw_grid<C>(grid: &Grid<C>, frame: &mut Frame)
where
    C: Cell,
{
    let pixels = frame.pixels();
    assert_eq!(grid.height(), pixels.height());
    assert_eq!(grid.width(), pixels.width());

    for v in grid.iter_points() {
        pixels.set(&v, grid.get(&v).color());
    }
}

impl<T> Game for CellUniverse<T>
where
    T: Cell,
{
    fn tick(&mut self) {
        for p in self.grid.iter_points() {
            let neighbors = [
                self.get(&(p + V::new(-1, 0))).clone(),
                self.get(&(p + V::new(1, 0))).clone(),
                self.get(&(p + V::new(0, -1))).clone(),
                self.get(&(p + V::new(0, 1))).clone(),
            ];
            let cell = self.get_mut(&p);
            cell.tick(&neighbors);
        }
    }

    fn render(&self, frame: &mut crate::display::Frame) {
        draw_grid(&self.grid, frame);
    }

    fn render_height(&self) -> usize {
        self.grid.height()
    }

    fn render_width(&self) -> usize {
        self.grid.width()
    }
}
