use std::marker::PhantomData;
use std::rc::Rc;

use wasm_retro_gamekit::display::Color;
use wasm_retro_gamekit::game::Painter;
use wasm_retro_gamekit::graphics::{
    Layer,
    PColor,
    Palette,
    PaletteRef,
    Scene,
    Sprite,
    SpritePixels,
    SpritePixelsRef,
};
use wasm_retro_gamekit::vector::v2::V2;

use super::cell::Cell;
use super::universe::Universe;

pub struct CellAutomataPainter<T> {
    t: PhantomData<T>,
    palette: PaletteRef,
}

impl<T> CellAutomataPainter<T> {
    pub fn new() -> Self {
        Self {
            t: PhantomData,
            palette: Rc::new(Palette::new(&[Color::rgb(60, 120, 60); 15])),
        }
    }
}

impl<T> CellAutomataPainter<T>
where
    T: Cell,
{
    fn make_cell_image(&self, color: PColor) -> SpritePixelsRef {
        SpritePixels::uniform(2, 2, color)
    }
    fn paint_cell(&self, vc: V2<i64>, state: &Universe<T>, scene: &mut Scene) {
        let grid = state.grid();
        let cell = grid.get(vc);
        let image = self.make_cell_image(cell.color());
        scene.add_sprite(Sprite::new(vc * 2, Layer::L0, image, self.palette.clone()));
    }
}

impl<T> Painter<Universe<T>> for CellAutomataPainter<T>
where
    T: Cell,
{
    fn paint(&self, state: &Universe<T>) -> Scene {
        let mut scene = Scene::new(self.scene_width(state), self.scene_height(state));

        let grid = state.grid();
        for v in grid.iter_v() {
            self.paint_cell(v, state, &mut scene);
        }
        scene
    }

    fn scene_height(&self, state: &Universe<T>) -> usize {
        state.grid().height() * 2
    }

    fn scene_width(&self, state: &Universe<T>) -> usize {
        state.grid().width() * 2
    }
}
