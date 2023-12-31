pub mod hex;

use std::iter::FromIterator;

use crate::vector::v2::V2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorRange {
    from_x: i64,
    to_x: i64,
    x: i64,
    from_y: i64,
    to_y: i64,
    y: i64,
}

impl VectorRange {
    pub fn new(from_x: i64, to_x: i64, from_y: i64, to_y: i64) -> Self {
        Self {
            from_x,
            to_x,
            x: from_x,
            from_y,
            to_y,
            y: from_y,
        }
    }
}

impl Iterator for VectorRange {
    type Item = V2<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.to_y {
            return None;
        }
        let p = V2::new(self.x, self.y);
        self.x += 1;
        if self.x >= self.to_x && self.y < self.to_y {
            self.x = self.from_x;
            self.y += 1;
        }
        Some(p)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
    wrapping: bool,
}

// TODO: split into base grid, wrapping grid
impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![T::default(); width * height],
            wrapping: true,
        }
    }
}

impl<T> Grid<T> {
    pub fn nowrap(&mut self) {
        self.wrapping = false;
    }
    pub fn reshape(&mut self, width: usize, height: usize) {
        if width * height != self.width * self.height {
            panic!("reshape to different size");
        }
        self.width = width;
        self.height = height;
    }
    fn wrap(&self, v: V2<i64>) -> V2<i64> {
        let w = self.width as i64;
        let h = self.height as i64;
        V2 {
            x: (w + v.x) % w,
            y: (h + v.y) % h,
        }
    }
    fn index(&self, v: V2<i64>) -> usize {
        let v = if self.wrapping { self.wrap(v) } else { v };
        (v.y * self.width as i64 + v.x) as usize
    }
    pub fn get(&self, v: V2<i64>) -> &T {
        &self.cells[self.index(v)]
    }
    pub fn get_mut(&mut self, v: V2<i64>) -> &mut T {
        let idx = self.index(v);
        &mut self.cells[idx]
    }
    pub fn mut_cells(&mut self) -> &mut Vec<T> {
        &mut self.cells
    }
    pub fn get_neighbors(&self, v: V2<i64>) -> [&T; 4] {
        [
            self.get(v + V2::new(0, -1)),
            self.get(v + V2::new(-1, 0)),
            self.get(v + V2::new(1, 0)),
            self.get(v + V2::new(0, 1)),
        ]
    }
    pub fn replace(&mut self, v: V2<i64>, value: T) -> T {
        let idx = self.index(v);
        std::mem::replace(&mut self.cells[idx], value)
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn iter_v(&self) -> impl Iterator<Item = V2<i64>> {
        VectorRange::new(0, self.width as i64, 0, self.height as i64)
    }
}

impl<T> FromIterator<T> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let cells: Vec<T> = iter.into_iter().collect();
        Self {
            width: cells.len(),
            height: 1,
            cells,
            wrapping: true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_ops() {
        let p1 = V2::new(2, 5);
        let p2 = V2::new(1, 3);

        assert_eq!(p1 + p2, V2::new(3, 8));
        assert_eq!(p1 - p2, V2::new(1, 2));
        assert_eq!(p1 * 2, V2::new(4, 10));
        assert_eq!(p2 / 2, V2::new(0, 1));
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid: Grid<u32> = Grid::new(10, 10);
        let v = V2::new(1, 2);
        let wv = V2::new(11, 12);
        assert_eq!(*grid.get(v), 0);
        assert_eq!(grid.replace(v, 222), 0);
        assert_eq!(*grid.get(wv), 222);
    }

    #[test]
    fn test_grid_reshape() {
        let mut grid: Grid<u32> = vec![0, 1, 2, 3, 4, 5].into_iter().collect();
        grid.reshape(3, 2);
        assert_eq!(*grid.get(V2::new(0, 1)), 3);
    }

    #[test]
    fn test_grid_iter_cells() {
        let mut grid: Grid<u32> = vec![0, 1, 2, 3, 4, 5].into_iter().collect();
        grid.reshape(3, 2);

        let pv: Vec<V2<i64>> = vec![V2::new(0, 0), V2::new(1, 1)];
        let vals: Vec<u32> = pv.into_iter().map(|v| grid.get(v)).cloned().collect();
        assert_eq!(vals, vec![0, 4]);
    }

    #[test]
    fn test_point_range() {
        let pr = VectorRange::new(1, 3, 2, 4);
        let vals: Vec<V2<i64>> = pr.collect();
        assert_eq!(
            vals,
            vec![V2::new(1, 2), V2::new(2, 2), V2::new(1, 3), V2::new(2, 3)]
        )
    }
}
