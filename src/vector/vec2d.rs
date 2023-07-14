use std::ops::{Add, Div, Mul, Sub};

use num::Zero;

use crate::physics::box2d::Float;

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vec2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2d<T> {
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

impl<T> Vec2d<T>
where
    T: Float,
{
    pub fn mag(&self) -> T {
        T::sqrt(self.x.powi(2) + self.y.powi(2))
    }
}

impl<T> Vec2d<T>
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

impl<T> Add for Vec2d<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vec2d<T>;

    fn add(self, rhs: Vec2d<T>) -> Self::Output {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2d<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Vec2d<T>) -> Self::Output {
        Vec2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2d<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for Vec2d<T>
where
    T: Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
