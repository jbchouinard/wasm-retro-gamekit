use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::num::{Float, One, Zero};

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V2<T> {
    pub x: T,
    pub y: T,
}

impl<T> V2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn ax(&self, axis: Axis) -> &T {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
        }
    }
    pub fn ax_mut(&mut self, axis: Axis) -> &mut T {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
        }
    }
}

impl<T> V2<T>
where
    T: Float,
{
    pub fn mag(&self) -> T {
        T::sqrt(self.x.powi(2) + self.y.powi(2))
    }

    pub fn norm(&self) -> Self {
        let mag = self.mag();
        if mag == T::zero() {
            Self::zero()
        } else {
            *self / mag
        }
    }

    pub fn round(&self) -> V2<i64> {
        V2 {
            x: self.x.round().to_i64().unwrap(),
            y: self.y.round().to_i64().unwrap(),
        }
    }
}

impl<T> V2<T>
where
    T: Zero,
{
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T> V2<T>
where
    T: One + Zero + Neg<Output = T>,
{
    pub fn unit(d: &Direction) -> Self {
        match d {
            Direction::Up => Self::new(T::zero(), -T::one()),
            Direction::Down => Self::new(T::zero(), T::one()),
            Direction::Left => Self::new(-T::one(), T::zero()),
            Direction::Right => Self::new(T::one(), T::zero()),
        }
    }
}

impl<T> Add for V2<T>
where
    T: Add<T, Output = T>,
{
    type Output = V2<T>;

    fn add(self, rhs: V2<T>) -> Self::Output {
        V2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for V2<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: V2<T>) -> Self::Output {
        V2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for V2<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        V2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for V2<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        V2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
