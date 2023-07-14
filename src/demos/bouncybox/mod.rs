mod input;

use std::{cell::RefCell, rc::Rc};

use crate::{
    display::Color,
    event::EventSource,
    game::{Game, Response},
    graphics::{Layer, PColor, Palette, PaletteRef, Scene, Sprite, SpriteImage, SpriteImageRef},
    input::{InputState, InputStateListener},
    physics::box2d::{dpad_v, Collider, HitBox, Mass, Mov, Object, ObjectId},
    vector::vec2d::Vec2d as V,
};

use self::input::{keymap, Keys};

pub struct BouncyBoxWorld {
    width: usize,
    height: usize,
    player: Rectangle,
    objects: Vec<Rectangle>,
    collider: Collider<f32>,
    last_t: f32,
    events: Option<EventSource>,
    input: Rc<RefCell<InputState>>,
    palette: PaletteRef,
}

impl BouncyBoxWorld {
    pub fn new(width: usize, height: usize, cor: f32, palette: PaletteRef) -> Self {
        let mut input = InputState::new();
        input.set_keymap(keymap());
        let size = (height + width) / 40;
        let padding = size;
        let mut this = Self {
            width,
            height,
            last_t: 0.0,
            player: player(size, width / 2, height / 2),
            objects: Vec::new(),
            collider: Collider::new(cor.clamp(-1.01, 1.01)),
            events: None,
            input: Rc::new(RefCell::new(input)),
            palette,
        };
        this.add_outer_walls();
        for y in (padding..(height - size - padding)).step_by(size + padding) {
            for x in (padding..(width - size - padding)).step_by(size + padding) {
                this.objects.push(square(size, x, y, 1.0, PColor::C2));
            }
        }
        this
    }

    fn add_outer_walls(&mut self) {
        let t = 1;
        let w = self.width;
        let h = self.height;
        let l = Layer::L0;
        let c = PColor::C8;
        self.objects.push(wall(0.0, 0.0, w, t, c, l));
        self.objects.push(wall(0.0, (h - t) as f32, w, t, c, l));
        self.objects.push(wall(0.0, t as f32, t, h - 2 * t, c, l));
        self.objects
            .push(wall((w - t) as f32, t as f32, t, h - 2 * t, c, l));
    }

    fn update_player_accel(&mut self) {
        let input = self.input.borrow();
        self.player.hitbox.mov.acc = dpad(&input) * 0.0005;
    }

    fn move_objects(&mut self, delta_t: f32) {
        self.update_player_accel();

        let mut objects: Vec<&mut dyn Object<f32>> = vec![&mut self.player];
        for obj in self.objects.iter_mut() {
            objects.push(obj);
        }
        self.collider.collide(&mut objects);

        for obj in objects.iter_mut() {
            obj.hitbox_mut().mov.update(delta_t);
        }
    }
}

fn palette() -> PaletteRef {
    Rc::new(Palette::new([
        Color::rgb(20, 100, 100),
        Color::rgb(100, 20, 100),
        Color::rgb(100, 100, 20),
        Color::rgb(0, 0, 0),
        Color::rgb(0, 0, 0),
        Color::rgb(0, 0, 0),
        Color::rgb(0, 0, 0),
        Color::rgb(0, 0, 0),
    ]))
}

fn player(side: usize, x: usize, y: usize) -> Rectangle {
    Rectangle::new(
        side,
        side,
        Mov {
            pos: V::new(x as f32, y as f32),
            vel: V::zero(),
            acc: V::zero(),
        },
        Mass::Density(1.0),
        PColor::C1,
        PColor::C8,
        Layer::L1,
    )
}

fn square(side: usize, x: usize, y: usize, density: f32, color: PColor) -> Rectangle {
    Rectangle::new(
        side,
        side,
        Mov {
            pos: V::new(x as f32, y as f32),
            vel: V::zero(),
            acc: V::zero(),
        },
        Mass::Density(100.0),
        color,
        PColor::C8,
        Layer::L1,
    )
}

fn wall(x: f32, y: f32, width: usize, height: usize, color: PColor, layer: Layer) -> Rectangle {
    let mov: Mov<f32> = Mov {
        pos: V::new(x, y),
        vel: V::new(0.0, 0.0),
        acc: V::new(0.0, 0.0),
    };
    Rectangle::new(width, height, mov, Mass::Infinite, color, color, layer)
}

fn dpad(input: &InputState) -> V<f32> {
    dpad_v::<f32>(
        input.is_key_pressed(Keys::Up),
        input.is_key_pressed(Keys::Down),
        input.is_key_pressed(Keys::Left),
        input.is_key_pressed(Keys::Right),
    )
}

impl Game for BouncyBoxWorld {
    fn start(&mut self, now: f32, events: &mut EventSource) {
        self.last_t = now;
        InputStateListener::new(self.input.clone()).listen(events);
    }

    fn tick(&mut self, now: f32) -> Response {
        if let Some(events) = &mut self.events {
            events.dispatch();
        }
        let delta_t = now - self.last_t;
        self.last_t = now;
        self.move_objects(delta_t);
        Response::RequestRedraw
    }

    fn paint(&self) -> Scene {
        let mut scene = Scene::new(self.width, self.height);
        scene.set_bg_color(Color::rgb(200, 200, 200));
        scene.add_sprite(self.player.sprite(self.palette.clone()));
        for obj in self.objects.iter() {
            scene.add_sprite(obj.sprite(self.palette.clone()));
        }
        scene
    }

    fn scene_width(&self) -> usize {
        self.width
    }

    fn scene_height(&self) -> usize {
        self.height
    }
}

pub fn bouncy_box_game(width: usize, height: usize, cor: f32) -> impl Game {
    BouncyBoxWorld::new(width, height, cor, palette())
}

pub struct Rectangle {
    id: ObjectId,
    hitbox: HitBox<f32>,
    layer: Layer,
    image: SpriteImageRef,
}

fn rectangle_image(width: usize, height: usize, fill: PColor, outline: PColor) -> SpriteImageRef {
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
    Rc::new(SpriteImage::new(image, width, height))
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
    pub fn sprite(&self, palette: PaletteRef) -> Sprite {
        Sprite::new(
            self.hitbox.mov.pixel_pos(),
            self.layer,
            self.image.clone(),
            palette,
        )
    }
}

impl Object<f32> for Rectangle {
    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn hitbox(&self) -> &HitBox<f32> {
        &self.hitbox
    }

    fn hitbox_mut(&mut self) -> &mut HitBox<f32> {
        &mut self.hitbox
    }
}
