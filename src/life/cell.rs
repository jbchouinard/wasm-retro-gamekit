use rand::Rng;

use crate::canvas::{Color, HasColor};

pub trait Cell: Default + Clone + HasColor + Sized {
    fn tick(&mut self, neighbors: &[Self]);
    fn randomize(&mut self, density: f32);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConwayCell {
    #[default]
    Dead = 0,
    Alive = 1,
}

impl HasColor for ConwayCell {
    fn has_color(&self) -> crate::canvas::Color {
        match self {
            ConwayCell::Alive => Color::rgb(0, 0, 0),
            ConwayCell::Dead => Color::rgb(255, 255, 255),
        }
    }
}

impl Cell for ConwayCell {
    fn tick(&mut self, neighbors: &[Self]) {
        let is_alive = matches!(self, ConwayCell::Alive);
        let live_neighbors: u8 = neighbors.iter().map(|c| *c as u8).sum();
        *self = match (is_alive, live_neighbors) {
            (true, 2) => ConwayCell::Alive,
            (_, 3) => ConwayCell::Alive,
            (_, _) => ConwayCell::Dead,
        }
    }

    fn randomize(&mut self, density: f32) {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(0.0..1.0);
        *self = match x < density {
            true => ConwayCell::Alive,
            false => ConwayCell::Dead,
        };
    }
}
