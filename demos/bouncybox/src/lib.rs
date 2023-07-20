mod input;
pub mod js;

use std::cmp::Ordering;

use input::{dpad, keymap};
use warg::display::Renderer;
use warg::event::{Events, MouseButton};
use warg::game::{Game, Response};
use warg::graphics::color::{Cm4, ColorMap4, Rgba32};
use warg::graphics::{parametric, Layer, Paint, Sprite, SpriteImage, SpriteImageRef, Viewport};
use warg::input::keyboard::{attach_keyboard, Keyboard};
use warg::input::mouse::{attach_mouse, Mouse, MouseInteractionKind};
use warg::input::Dpad;
use warg::physics::box2d::{Box2DPhysics, HitBox, Mass, Mov, Object, ObjectId};
use warg::physics::identity::{Identity, ObjectKey};
use warg::physics::universe::{Space, Universe};
use warg::vector::v2::V2;

use self::input::Keys;

pub struct BouncyBoxWorld {
    scale: usize,
    universe: Universe<Box2DPhysics<f32>, Rectangle>,
    player_key: ObjectKey,
    viewport: Viewport,
    dpad: Dpad<Keys>,
    keyboard: Option<Keyboard<Keys>>,
    mouse: Option<Mouse>,
    last_t: f32,
    background: Option<SpriteImageRef>,
    drag_start: V2<i64>,
}

impl BouncyBoxWorld {
    pub fn new(width: usize, height: usize, cor: f32) -> Self {
        let viewport = Viewport::new(
            V2::new(-(width as i64) / 2, -(height as i64) / 2),
            width,
            height,
        );

        let mut universe = Universe::new(Box2DPhysics::new(cor));
        let space = universe.space_mut();
        add_outer_walls(space, width - 50, height - 50, 1_000_000);

        let scale = (height + width) / 40;
        let player = bouncybox(scale, scale, V2::new(0, 0), 1.0, Cm4::C1);
        let player_key = space.add(player);

        Self {
            scale,
            last_t: 0.0,
            keyboard: None,
            mouse: None,
            dpad: dpad(),
            player_key,
            viewport,
            universe,
            background: Some(background()),
            drag_start: V2::zero(),
        }
    }

    fn process_mouse_interactions(&mut self) {
        let mut clicks: Vec<(V2<f32>, MouseButton)> = vec![];
        let mut drags: Vec<(V2<f32>, MouseButton)> = vec![];
        let mut drops: Vec<(V2<f32>, MouseButton)> = vec![];
        if let Some(mouse) = &self.mouse {
            let interactions = mouse.interactions();
            while let Some(event) = interactions.recv() {
                match event.kind {
                    MouseInteractionKind::Click => {
                        clicks.push((event.pos, event.button));
                    },
                    MouseInteractionKind::Drag => {
                        drags.push((event.pos, event.button));
                    },
                    MouseInteractionKind::Drop => {
                        drops.push((event.pos, event.button));
                    },
                }
            }
        }
        for (pos, button) in clicks {
            self.on_mouse_click(pos, button);
        }
        for (pos, button) in drags {
            self.on_mouse_drag(pos, button);
        }
        for (pos, button) in drops {
            self.on_mouse_drop(pos, button)
        }
    }

    fn on_mouse_drag(&mut self, pos: V2<f32>, button: MouseButton) {
        if let MouseButton::Left = button {
            self.drag_start = self.viewport.relative_pos(pos)
        }
    }

    fn on_mouse_drop(&mut self, pos: V2<f32>, button: MouseButton) {
        if let MouseButton::Left = button {
            let drag_end = self.viewport.relative_pos(pos);
            let (x_min, x_max) = match self.drag_start.x.cmp(&drag_end.x) {
                Ordering::Less => (self.drag_start.x, drag_end.x),
                _ => (drag_end.x, self.drag_start.x),
            };
            let (y_min, y_max) = match self.drag_start.y.cmp(&drag_end.y) {
                Ordering::Less => (self.drag_start.y, drag_end.y),
                _ => (drag_end.y, self.drag_start.y),
            };

            let width = x_max - x_min;
            let height = y_max - y_min;
            let pos = V2::new(x_min, y_min);
            self.universe.space_mut().add(bouncybox(
                width as usize,
                height as usize,
                pos,
                0.5,
                Cm4::C3,
            ));
        }
    }

    fn on_mouse_click(&mut self, pos: V2<f32>, button: MouseButton) {
        match button {
            MouseButton::Left => {
                let size = self.scale;
                let center_pos = self.viewport.relative_pos(pos);
                let tl_pos = center_pos - V2::new((size as i64) / 2, (size as i64) / 2);
                self.universe
                    .space_mut()
                    .add(bouncybox(size, size, tl_pos, 1.0, Cm4::C2));
            },
            MouseButton::Right => {
                let size = self.scale;
                let center_pos = self.viewport.relative_pos(pos);
                self.universe.space_mut().add(wall(center_pos, size, size));
            },
            _ => (),
        }
    }

    fn update_player_accel(&mut self) {
        if let Some(input) = &self.keyboard {
            let space = self.universe.space_mut();
            let player = space.get_mut(self.player_key).unwrap();
            player.hitbox.mov.acc = self.dpad.read(input) * 0.001;
        }
    }
}

impl Game for BouncyBoxWorld {
    fn start(&mut self, now: f32, events: &mut Events) {
        self.last_t = now;
        self.keyboard = Some(attach_keyboard(events, keymap()));
        self.mouse = Some(attach_mouse(events, 200.0, 0.02));
    }

    fn tick(&mut self, now: f32) -> Response {
        if let Some(kb) = &mut self.keyboard {
            kb.update();
        }
        self.process_mouse_interactions();
        self.update_player_accel();
        self.universe.tick(now - self.last_t);
        self.last_t = now;
        Response::RequestRedraw
    }

    fn renderer(&self) -> Box<dyn Renderer> {
        let mut scene = self.universe.space().paint(&self.viewport);
        scene.set_background(self.background.clone());
        Box::new(scene)
    }

    fn scene_width(&self) -> usize {
        self.viewport.width
    }

    fn scene_height(&self) -> usize {
        self.viewport.height
    }
}

pub fn bouncy_box_world(width: usize, height: usize, cor: f32) -> impl Game {
    BouncyBoxWorld::new(width, height, cor)
}

fn background() -> SpriteImageRef {
    SpriteImage::parametric(
        30,
        30,
        default_palette(),
        parametric::Aspect::Stretch,
        parametric::tile(Cm4::C6, Cm4::C7),
    )
}

fn default_palette() -> ColorMap4 {
    ColorMap4::new(&[
        Rgba32::rgba(0, 0, 0, 0),
        Rgba32::rgb(200, 40, 40),
        Rgba32::rgb(40, 40, 200),
        Rgba32::rgb(20, 100, 100),
        Rgba32::rgb(100, 20, 100),
        Rgba32::rgb(100, 100, 20),
        Rgba32::rgb(165, 165, 165),
        Rgba32::rgb(225, 225, 225),
        Rgba32::rgb(64, 64, 64),
    ])
}

fn bouncybox(width: usize, height: usize, pos: V2<i64>, density: f32, color: Cm4) -> Rectangle {
    Rectangle::new(
        width,
        height,
        Mov {
            pos: V2::new(pos.x as f32, pos.y as f32),
            vel: V2::zero(),
            acc: V2::zero(),
        },
        Mass::Density(density),
        color,
        Cm4::C7,
        Layer::L1,
    )
}

fn wall(center: V2<i64>, width: usize, height: usize) -> Rectangle {
    let tl = V2::new(
        center.x - (width as i64) / 2,
        center.y - (height as i64) / 2,
    );
    let mov: Mov<f32> = Mov {
        pos: V2::new(tl.x as f32, tl.y as f32),
        vel: V2::new(0.0, 0.0),
        acc: V2::new(0.0, 0.0),
    };
    Rectangle::new(
        width,
        height,
        mov,
        Mass::Infinite,
        Cm4::C8,
        Cm4::C8,
        Layer::L7,
    )
}

fn add_outer_walls(space: &mut Space<Rectangle>, width: usize, height: usize, t: i64) {
    let h = (height as i64) / 2;
    let w = (width as i64) / 2;
    let tdw = (2 * w + 2 * t) as usize;
    let lrh = (2 * h) as usize;
    space.add(wall(V2::new(0, -(h + t / 2)), tdw, t as usize));
    space.add(wall(V2::new(0, h + t / 2), tdw, t as usize));
    space.add(wall(V2::new(-(w + t / 2), 0), t as usize, lrh));
    space.add(wall(V2::new(w + t / 2, 0), t as usize, lrh));
}

pub struct Rectangle {
    id: ObjectId,
    hitbox: HitBox<f32>,
    layer: Layer,
    image: SpriteImageRef,
}

impl Rectangle {
    fn new(
        width: usize,
        height: usize,
        mov: Mov<f32>,
        mass: Mass<f32>,
        fill_color: Cm4,
        outline_color: Cm4,
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
    fn paint(&self) -> Option<Sprite> {
        Some(Sprite::new(
            self.hitbox.mov.pos.round(),
            self.layer,
            self.image.clone(),
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

fn rectangle_image(width: usize, height: usize, fill: Cm4, outline: Cm4) -> SpriteImageRef {
    if fill == outline {
        SpriteImage::monochrome(width, height, default_palette().map_color(fill))
    } else {
        SpriteImage::parametric(
            width,
            height,
            default_palette(),
            parametric::Aspect::Stretch,
            parametric::rectangle(fill, outline, 0.0),
        )
    }
}
