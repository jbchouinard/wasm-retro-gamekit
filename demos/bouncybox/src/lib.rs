mod input;
pub mod js;

use std::rc::Rc;

use input::dpad;
use wasm_retro_gamekit::{
    display::Color,
    event::{Events, MouseButton, MouseEvent, MouseEventKind, Source},
    game::{Game, Response},
    graphics::{
        Layer, PColor, Paint, Palette, PaletteRef, Scene, Sprite, SpritePixels, SpritePixelsRef,
    },
    input::{Dpad, InputState},
    physics::{
        box2d::{Box2DPhysics, HitBox, Mass, Mov, Object, ObjectId},
        identity::{Identity, ObjectKey},
        universe::{Space, Universe, Viewport},
    },
    vector::vec2d::Vec2d as V,
};

use self::input::{inputs, Keys};

pub struct BouncyBoxWorld {
    scale: usize,
    universe: Universe<Box2DPhysics<f32>, Rectangle>,
    player_key: ObjectKey,
    viewport: Viewport,
    input: Option<InputState>,
    dpad: Dpad<Keys>,
    mouse: Option<Source<MouseEvent>>,
    last_t: f32,
    palette: PaletteRef,
}

impl BouncyBoxWorld {
    pub fn new(width: usize, height: usize, cor: f32, palette: PaletteRef) -> Self {
        let viewport = Viewport::new(
            V::new(-(width as i64) / 2, -(height as i64) / 2),
            width,
            height,
        );

        let mut universe = Universe::new(Box2DPhysics::new(cor));
        let space = universe.space_mut();
        add_outer_walls(space, width - 50, height - 50, 1_000_000);

        let scale = (height + width) / 40;
        let player = square(scale, V::new(0, 0), 1.0, PColor::C1);
        let player_key = space.add(player);

        Self {
            scale,
            last_t: 0.0,
            input: None,
            mouse: None,
            dpad: dpad(),
            player_key,
            viewport,
            universe,
            palette,
        }
    }

    fn recv_mouse(&mut self) {
        let mut clicks: Vec<(V<f32>, MouseButton)> = vec![];
        if let Some(mouse) = &self.mouse {
            while let Some(event) = mouse.recv() {
                if let MouseEventKind::Down(button) = event.kind {
                    clicks.push((event.pos, button));
                }
            }
        }
        for (pos, button) in clicks {
            self.on_mouse_down(pos, button);
        }
    }

    fn on_mouse_down(&mut self, pos: V<f32>, button: MouseButton) {
        if let MouseButton::Right = button {
            let size = self.scale;
            let center_pos = self.viewport.relative_pos(pos);
            let tl_pos = center_pos - V::new((size as i64) / 2, (size as i64) / 2);
            self.universe
                .space_mut()
                .add(square(size, tl_pos, 1.0, PColor::C2));
        }
    }

    fn update_player_accel(&mut self) {
        if let Some(input) = &self.input {
            let space = self.universe.space_mut();
            let player = space.get_mut(self.player_key).unwrap();
            player.hitbox.mov.acc = self.dpad.read(input) * 0.001;
        }
    }
}

impl Game for BouncyBoxWorld {
    fn start(&mut self, now: f32, events: &mut Events) {
        self.last_t = now;
        self.input = Some(inputs(events.key_events()));
        self.mouse = Some(events.mouse_events());
    }

    fn tick(&mut self, now: f32) -> Response {
        if let Some(input) = &mut self.input {
            input.update();
        }
        self.recv_mouse();
        self.update_player_accel();
        self.universe.tick(now - self.last_t);
        self.last_t = now;
        Response::RequestRedraw
    }

    fn paint(&self) -> Scene {
        let mut scene = self
            .universe
            .space()
            .paint(&self.viewport, self.palette.clone());
        scene.set_bg_color(Color::rgb(200, 200, 200));
        scene
    }

    fn scene_width(&self) -> usize {
        self.viewport.width
    }

    fn scene_height(&self) -> usize {
        self.viewport.height
    }
}

pub fn bouncy_box_game(width: usize, height: usize, cor: f32) -> impl Game {
    BouncyBoxWorld::new(width, height, cor, default_palette())
}

fn default_palette() -> PaletteRef {
    Rc::new(Palette::new([
        Color::rgb(200, 40, 40),
        Color::rgb(40, 40, 200),
        Color::rgb(20, 100, 100),
        Color::rgb(100, 20, 100),
        Color::rgb(100, 100, 20),
        Color::rgb(40, 200, 200),
        Color::rgb(255, 255, 255),
        Color::rgb(0, 0, 0),
    ]))
}

fn square(side: usize, pos: V<i64>, density: f32, color: PColor) -> Rectangle {
    Rectangle::new(
        side,
        side,
        Mov {
            pos: V::new(pos.x as f32, pos.y as f32),
            vel: V::zero(),
            acc: V::zero(),
        },
        Mass::Density(density),
        color,
        PColor::C8,
        Layer::L1,
    )
}

fn wall(center: V<i64>, width: usize, height: usize) -> Rectangle {
    let tl = V::new(
        center.x - (width as i64) / 2,
        center.y - (height as i64) / 2,
    );
    let mov: Mov<f32> = Mov {
        pos: V::new(tl.x as f32, tl.y as f32),
        vel: V::new(0.0, 0.0),
        acc: V::new(0.0, 0.0),
    };
    Rectangle::new(
        width,
        height,
        mov,
        Mass::Infinite,
        PColor::C8,
        PColor::C8,
        Layer::L7,
    )
}

fn add_outer_walls(space: &mut Space<Rectangle>, width: usize, height: usize, t: i64) {
    let h = (height as i64) / 2;
    let w = (width as i64) / 2;
    let tdw = (2 * w + 2 * t) as usize;
    let lrh = (2 * h) as usize;
    space.add(wall(V::new(0, -(h + t / 2)), tdw, t as usize));
    space.add(wall(V::new(0, h + t / 2), tdw, t as usize));
    space.add(wall(V::new(-(w + t / 2), 0), t as usize, lrh));
    space.add(wall(V::new(w + t / 2, 0), t as usize, lrh));
}

pub struct Rectangle {
    id: ObjectId,
    hitbox: HitBox<f32>,
    layer: Layer,
    image: SpritePixelsRef,
}

impl Rectangle {
    fn new(
        width: usize,
        height: usize,
        mov: Mov<f32>,
        mass: Mass<f32>,
        fill_color: PColor,
        outline_color: PColor,
        layer: Layer,
    ) -> Self {
        Self {
            id: ObjectId::new(),
            hitbox: HitBox {
                width,
                height,
                mov,
                mass,
            },
            image: rectangle_image(width, height, fill_color, outline_color),
            layer,
        }
    }
    pub fn width(&self) -> usize {
        self.hitbox.width
    }
    pub fn height(&self) -> usize {
        self.hitbox.height
    }
}

impl Paint for Rectangle {
    fn paint(&self, palette: PaletteRef) -> Option<Sprite> {
        Some(Sprite::new(
            self.hitbox.mov.pos.round(),
            self.layer,
            self.image.clone(),
            palette,
        ))
    }
}

impl Identity for Rectangle {
    fn id(&self) -> &ObjectId {
        &self.id
    }
}

impl Object<f32> for Rectangle {
    fn hitbox(&self) -> &HitBox<f32> {
        &self.hitbox
    }

    fn hitbox_mut(&mut self) -> &mut HitBox<f32> {
        &mut self.hitbox
    }
}

fn rectangle_image(width: usize, height: usize, fill: PColor, outline: PColor) -> SpritePixelsRef {
    if fill == outline {
        Rc::new(SpritePixels::uniform(width, height, fill))
    } else {
        let mut image: Vec<PColor> = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                image.push(if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    outline
                } else {
                    fill
                })
            }
        }
        Rc::new(SpritePixels::image(width, height, image))
    }
}
