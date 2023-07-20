use std::marker::PhantomData;

use warg::display::Renderer;
use warg::event::Events;
use warg::game::{Game, Response};
use warg::graphics::color::Rgba32;
use warg::graphics::{Layer, Scene, Sprite, SpriteImage, SpriteImageRef};
use warg::vector::v2::V2;

use super::cell::Cell;
use super::universe::Universe;

pub struct CellAutomataWorld<T: Cell> {
    state: Universe<T>,
    last_generation_ts: f32,
    generation_interval: f32,
    t: PhantomData<T>,
}

impl<T: Cell> CellAutomataWorld<T> {
    pub fn new(width: usize, height: usize, density: f32, generation_interval: f32) -> Self {
        let mut state = Universe::new(width, height);
        state.randomize(density);
        Self {
            state,
            generation_interval,
            last_generation_ts: 0.0,
            t: PhantomData,
        }
    }

    fn make_cell_image(&self, color: Rgba32) -> SpriteImageRef {
        SpriteImage::monochrome(2, 2, color)
    }
    fn paint_cell(&self, vc: V2<i64>, state: &Universe<T>, scene: &mut Scene) {
        let grid = state.grid();
        let cell = grid.get(vc);
        let image = self.make_cell_image(cell.color());
        scene.add_sprite(Sprite::new(vc * 2, Layer::L0, image));
    }
}

impl<T: Cell> Game for CellAutomataWorld<T> {
    fn start(&mut self, _now: f32, _events: &mut Events) {}

    fn tick(&mut self, now: f32) -> Response {
        if (now - self.last_generation_ts) >= self.generation_interval {
            self.state.tick();
            self.last_generation_ts = now;
            Response::RequestRedraw
        } else {
            Response::Empty
        }
    }

    fn renderer(&self) -> Box<dyn Renderer> {
        let mut scene = Scene::new(self.scene_width(), self.scene_height());

        let grid = self.state.grid();
        for v in grid.iter_v() {
            self.paint_cell(v, &self.state, &mut scene);
        }
        Box::new(scene)
    }

    fn scene_width(&self) -> usize {
        self.state.grid().width() * 2
    }

    fn scene_height(&self) -> usize {
        self.state.grid().height() * 2
    }
}
