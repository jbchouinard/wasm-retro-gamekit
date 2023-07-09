mod canvas;
mod grid;
pub mod js;
mod life;

use rand::Rng;

use grid::{Grid, Point};

#[repr(u8)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum Cell {
    #[default]
    Dead = 0,
    Alive = 1,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Dead => '◻',
                Cell::Alive => '◼',
            }
        )
    }
}

pub struct Universe {
    grid: Grid<Cell>,
}

impl Universe {
    pub fn empty(width: u32, height: u32) -> Self {
        Self {
            grid: Grid::new(width, height),
        }
    }
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        Self { grid }
    }
    fn get(&self, p: &Point) -> &Cell {
        self.grid.get(p)
    }
    fn neighbors(&self, p: &Point) -> Vec<&Cell> {
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
            .collect()
    }
    fn live_neighbor_count(&self, p: &Point) -> u8 {
        self.neighbors(p).into_iter().map(|c| *c as u8).sum()
    }
}

impl Universe {
    pub fn new(width: u32, height: u32, density: u8) -> Self {
        let mut rng = rand::thread_rng();

        let mut grid: Grid<Cell> = (0..width * height)
            .map(|_| {
                if rng.gen_range(0..100) < density {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        grid.reshape(width, height);
        Self::from_grid(grid)
    }
    pub fn width(&self) -> u32 {
        self.grid.width()
    }
    pub fn height(&self) -> u32 {
        self.grid.height()
    }
    pub fn cells(&self) -> *const Cell {
        self.grid.cells()
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn tick(&mut self) {
        let mut updated_cells: Vec<Cell> = Vec::new();
        for point in self.grid.iter_points() {
            let cell = self.get(&point);
            let live_neighbors = self.live_neighbor_count(&point);
            updated_cells.push(match (cell, live_neighbors) {
                (Cell::Alive, 2) => Cell::Alive,
                (_, 3) => Cell::Alive,
                (_, _) => Cell::Dead,
            })
        }
        let mut updated_grid: Grid<Cell> = updated_cells.into_iter().collect();
        updated_grid.reshape(self.grid.width(), self.grid.height());
        self.grid = updated_grid;
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new(64, 64, 20)
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_y = 0;
        for p in self.grid.iter_points() {
            if p.y != current_y {
                writeln!(f)?;
                current_y = p.y;
            }
            write!(f, "{} ", self.get(&p))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use grid::Grid;

    #[test]
    fn test_universe() {
        let mut grid: Grid<Cell> = vec![
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ]
        .into_iter()
        .collect();
        grid.reshape(4, 4);

        let mut universe = Universe::from_grid(grid);
        assert_eq!(universe.live_neighbor_count(&Point::new(0, 0)), 0);
        assert_eq!(universe.live_neighbor_count(&Point::new(1, 1)), 2);

        let prev_grid = universe.grid.clone();
        universe.tick();
        assert_eq!(prev_grid, universe.grid);
    }
}
