//! Hex grid with double-width horizontal layout
//! https://www.redblobgames.com/grids/hexagons/

use std::iter::FromIterator;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Hv {
    col: i64,
    row: i64,
}

impl Hv {
    pub fn new(col: i64, row: i64) -> Self {
        assert!((col + row) % 2 == 0, "invalid double-width hex vector");
        Self { col, row }
    }
}

impl std::ops::Add for Hv {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Hv::new(self.col + rhs.col, self.row + rhs.row)
    }
}

#[derive(Debug)]
pub struct HvIter {
    col: i64,
    row: i64,
    width: i64,
    height: i64,
}

impl HvIter {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width: width as i64,
            height: height as i64,
            col: 0,
            row: 0,
        }
    }
}

impl Iterator for HvIter {
    type Item = Hv;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.height {
            return None;
        }
        let next_hv = Hv::new(self.col, self.row);

        self.col += 2;
        if self.col >= (2 * self.width) {
            self.row += 1;
            self.col = self.row % 2;
        }

        Some(next_hv)
    }
}

#[derive(Clone, Debug)]
pub struct HexGrid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> HexGrid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        assert!(height % 2 == 0, "must have even height");
        Self {
            width,
            height,
            cells: vec![T::default(); width * height],
        }
    }
}

impl<T> HexGrid<T> {
    fn wrap(&self, hv: Hv) -> Hv {
        let w = self.width as i64;
        let h = self.height as i64;
        let col = (hv.col + (2 * w)) % (2 * w);
        let row = (hv.row + h) % h;
        Hv::new(col, row)
    }
    fn index(&self, hv: Hv) -> usize {
        let hv = self.wrap(hv);
        let col = hv.col as usize;
        let row = hv.row as usize;
        row * self.width + col / 2
    }
    pub fn reshape(&mut self, width: usize, height: usize) {
        assert_eq!(
            width * height,
            self.width * self.height,
            "reshape out of bounds"
        );
        self.width = width;
        self.height = height;
    }
    pub fn get(&self, hv: Hv) -> &T {
        &self.cells[self.index(hv)]
    }
    pub fn get_mut(&mut self, hv: Hv) -> &mut T {
        let idx = self.index(hv);
        &mut self.cells[idx]
    }
    pub fn get_neighbors(&self, hv: Hv) -> [&T; 6] {
        [
            self.get(hv + Hv::new(-1, -1)),
            self.get(hv + Hv::new(1, -1)),
            self.get(hv + Hv::new(-2, 0)),
            self.get(hv + Hv::new(2, 0)),
            self.get(hv + Hv::new(-1, 1)),
            self.get(hv + Hv::new(1, 1)),
        ]
    }
    pub fn iter_hv(&self) -> impl Iterator<Item = Hv> {
        HvIter::new(self.width, self.height)
    }
}

impl<T> FromIterator<T> for HexGrid<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let cells: Vec<T> = iter.into_iter().collect();
        Self {
            width: cells.len(),
            height: 1,
            cells,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid_get() {
        let mut grid: HexGrid<u8> = [0, 1, 2, 3].iter().cloned().collect();
        grid.reshape(2, 2);

        assert_eq!(*grid.get(Hv::new(0, 0)), 0);
        assert_eq!(*grid.get(Hv::new(2, 0)), 1);
        assert_eq!(*grid.get(Hv::new(1, 1)), 2);
        assert_eq!(*grid.get(Hv::new(3, 1)), 3);
    }

    #[test]
    fn test_grid_iter_hv() {
        let grid: HexGrid<u8> = HexGrid::new(2, 2);
        let hvs: Vec<Hv> = grid.iter_hv().collect();

        assert_eq!(
            hvs,
            vec![Hv::new(0, 0), Hv::new(2, 0), Hv::new(1, 1), Hv::new(3, 1)]
        )
    }
}
