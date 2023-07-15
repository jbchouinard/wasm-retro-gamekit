use super::cell::Cell;
use wasm_retro_gamekit::grid::{Grid, Vector};

#[derive(Default)]
pub struct Universe<T: Cell> {
    grid: Grid<T>,
}

impl<T> Universe<T>
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

impl<T> Universe<T>
where
    T: Cell,
{
    fn get(&self, v: Vector) -> &T {
        self.grid.get(v)
    }
    fn get_mut(&mut self, v: Vector) -> &mut T {
        self.grid.get_mut(v)
    }
    pub fn randomize(&mut self, density: f32) {
        for v in self.grid.iter_v() {
            let cell = self.grid.get_mut(v);
            cell.randomize(density);
        }
    }
    pub fn grid(&self) -> &Grid<T> {
        &self.grid
    }
    pub fn tick(&mut self) {
        for v in self.grid.iter_v() {
            let neighbors = [
                self.get(v + Vector::new(-1, 0)).clone(),
                self.get(v + Vector::new(1, 0)).clone(),
                self.get(v + Vector::new(0, -1)).clone(),
                self.get(v + Vector::new(0, 1)).clone(),
            ];
            let cell = self.get_mut(v);
            cell.tick(&neighbors);
        }
    }
}
