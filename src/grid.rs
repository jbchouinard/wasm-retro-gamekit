use std::iter::FromIterator;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<i64> for Point {
    type Output = Point;

    fn div(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointRange {
    from_x: i64,
    to_x: i64,
    x: i64,
    from_y: i64,
    to_y: i64,
    y: i64,
}

impl PointRange {
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

impl Iterator for PointRange {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.to_y {
            return None;
        }
        let p = Point::new(self.x, self.y);
        self.x += 1;
        if self.x >= self.to_x && self.y < self.to_y {
            self.x = self.from_x;
            self.y += 1;
        }
        Some(p)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> {
    width: u32,
    height: u32,
    cells: Vec<T>,
}

pub struct GridPointsIterator<'a, T, I>
where
    I: Iterator<Item = Point>,
{
    grid: &'a Grid<T>,
    itp: I,
}

impl<'a, T, I> Iterator for GridPointsIterator<'a, T, I>
where
    I: Iterator<Item = Point>,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.itp.next() {
            Some(p) => Some(self.grid.get(&p)),
            None => None,
        }
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            cells: vec![T::default(); (width * height) as usize],
        }
    }
}

impl<T> Grid<T> {
    pub fn reshape(&mut self, width: u32, height: u32) {
        if width * height != self.width * self.height {
            panic!("reshape to different size");
        }
        self.width = width;
        self.height = height;
    }
    fn wrap(&self, p: &Point) -> Point {
        let w = self.width as i64;
        let h = self.height as i64;
        Point {
            x: (w + p.x) % w,
            y: (h + p.y) % h,
        }
    }
    fn index(&self, p: &Point) -> usize {
        let wp = self.wrap(p);
        (wp.y * self.width as i64 + wp.x) as usize
    }
    pub fn get(&self, p: &Point) -> &T {
        &self.cells[self.index(p)]
    }
    pub fn set(&mut self, p: &Point, v: T) -> T {
        let idx = self.index(p);
        std::mem::replace(&mut self.cells[idx], v)
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn iter_points(&self) -> impl Iterator<Item = Point> {
        PointRange::new(0, self.width as i64, 0, self.height as i64)
    }
    pub fn iter_from_points<I>(&self, itp: I) -> impl Iterator<Item = &T>
    where
        I: IntoIterator<Item = Point>,
    {
        GridPointsIterator {
            grid: self,
            itp: itp.into_iter(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.iter_from_points(self.iter_points())
    }
    pub fn cells(&self) -> *const T {
        self.cells.as_ptr()
    }
}

impl<T> FromIterator<T> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let cells: Vec<T> = iter.into_iter().collect();
        Self {
            width: cells.len() as u32,
            height: 1,
            cells,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_ops() {
        let p1 = Point::new(2, 5);
        let p2 = Point::new(1, 3);

        assert_eq!(p1 + p2, Point::new(3, 8));
        assert_eq!(p1 - p2, Point::new(1, 2));
        assert_eq!(p1 * 2, Point::new(4, 10));
        assert_eq!(p2 / 2, Point::new(0, 1));
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid: Grid<u32> = Grid::new(10, 10);
        let p = Point::new(1, 2);
        let wp = Point::new(11, 12);
        assert_eq!(*grid.get(&p), 0);
        assert_eq!(grid.set(&p, 222), 0);
        assert_eq!(*grid.get(&wp), 222);
    }

    #[test]
    fn test_grid_reshape() {
        let mut grid: Grid<u32> = vec![0, 1, 2, 3, 4, 5].into_iter().collect();
        grid.reshape(3, 2);
        assert_eq!(*grid.get(&Point::new(0, 1)), 3);
    }

    #[test]
    fn test_grid_iter_cells() {
        let mut grid: Grid<u32> = vec![0, 1, 2, 3, 4, 5].into_iter().collect();
        grid.reshape(3, 2);

        let pv_empty: Vec<Point> = vec![];
        let mut grid_iter_empty = grid.iter_from_points(pv_empty.into_iter());
        assert!(grid_iter_empty.next().is_none());

        let pv: Vec<Point> = vec![Point::new(0, 0), Point::new(1, 1)];
        let vals: Vec<u32> = grid.iter_from_points(pv.into_iter()).cloned().collect();
        assert_eq!(vals, vec![0, 4]);
    }

    #[test]
    fn test_point_range() {
        let pr = PointRange::new(1, 3, 2, 4);
        let vals: Vec<Point> = pr.collect();
        assert_eq!(
            vals,
            vec![
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(1, 3),
                Point::new(2, 3)
            ]
        )
    }
}
