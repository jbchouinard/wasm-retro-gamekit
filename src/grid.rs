use std::iter::FromIterator;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V {
    pub x: i64,
    pub y: i64,
}

impl V {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for V {
    type Output = V;

    fn add(self, rhs: V) -> Self::Output {
        V {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for V {
    type Output = V;

    fn sub(self, rhs: V) -> Self::Output {
        V {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for V {
    type Output = V;

    fn mul(self, rhs: i64) -> Self::Output {
        V {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<i64> for V {
    type Output = V;

    fn div(self, rhs: i64) -> Self::Output {
        V {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VRange {
    from_x: i64,
    to_x: i64,
    x: i64,
    from_y: i64,
    to_y: i64,
    y: i64,
}

impl VRange {
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

impl Iterator for VRange {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.to_y {
            return None;
        }
        let p = V::new(self.x, self.y);
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
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![T::default(); width * height],
        }
    }
}

impl<T> Grid<T> {
    pub fn reshape(&mut self, width: usize, height: usize) {
        if width * height != self.width * self.height {
            panic!("reshape to different size");
        }
        self.width = width;
        self.height = height;
    }
    fn wrap(&self, p: &V) -> V {
        let w = self.width as i64;
        let h = self.height as i64;
        V {
            x: (w + p.x) % w,
            y: (h + p.y) % h,
        }
    }
    fn index(&self, p: &V) -> usize {
        let wp = self.wrap(p);
        (wp.y * self.width as i64 + wp.x) as usize
    }
    pub fn get(&self, p: &V) -> &T {
        &self.cells[self.index(p)]
    }
    pub fn get_mut(&mut self, p: &V) -> &mut T {
        let idx = self.index(p);
        &mut self.cells[idx]
    }
    pub fn set(&mut self, p: &V, v: T) -> T {
        let idx = self.index(p);
        std::mem::replace(&mut self.cells[idx], v)
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn iter_points(&self) -> impl Iterator<Item = V> {
        VRange::new(0, self.width as i64, 0, self.height as i64)
    }
}

impl<T> FromIterator<T> for Grid<T> {
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
    fn test_point_ops() {
        let p1 = V::new(2, 5);
        let p2 = V::new(1, 3);

        assert_eq!(p1 + p2, V::new(3, 8));
        assert_eq!(p1 - p2, V::new(1, 2));
        assert_eq!(p1 * 2, V::new(4, 10));
        assert_eq!(p2 / 2, V::new(0, 1));
    }

    #[test]
    fn test_grid_get_set() {
        let mut grid: Grid<u32> = Grid::new(10, 10);
        let p = V::new(1, 2);
        let wp = V::new(11, 12);
        assert_eq!(*grid.get(&p), 0);
        assert_eq!(grid.set(&p, 222), 0);
        assert_eq!(*grid.get(&wp), 222);
    }

    #[test]
    fn test_grid_reshape() {
        let mut grid: Grid<u32> = vec![0, 1, 2, 3, 4, 5].into_iter().collect();
        grid.reshape(3, 2);
        assert_eq!(*grid.get(&V::new(0, 1)), 3);
    }

    #[test]
    fn test_grid_iter_cells() {
        let mut grid: Grid<u32> = vec![0, 1, 2, 3, 4, 5].into_iter().collect();
        grid.reshape(3, 2);

        let pv: Vec<V> = vec![V::new(0, 0), V::new(1, 1)];
        let vals: Vec<u32> = pv.iter().map(|p| grid.get(p)).cloned().collect();
        assert_eq!(vals, vec![0, 4]);
    }

    #[test]
    fn test_point_range() {
        let pr = VRange::new(1, 3, 2, 4);
        let vals: Vec<V> = pr.collect();
        assert_eq!(
            vals,
            vec![V::new(1, 2), V::new(2, 2), V::new(1, 3), V::new(2, 3)]
        )
    }
}
