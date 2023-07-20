use std::collections::HashMap;

use super::identity::{Identity, IdentityKey, ObjectKey};
use crate::graphics::{Paint, Scene, Viewport};

pub trait Physics<T> {
    fn tick(&mut self, space: &mut Space<T>, delta_t: f32);
}

pub struct Universe<P, T> {
    physics: P,
    space: Space<T>,
}

impl<P, T> Universe<P, T>
where
    T: Identity,
    P: Physics<T>,
{
    pub fn new(physics: P) -> Self {
        Self {
            physics,
            space: Space::new(),
        }
    }

    pub fn space(&self) -> &Space<T> {
        &self.space
    }

    pub fn space_mut(&mut self) -> &mut Space<T> {
        &mut self.space
    }

    pub fn tick(&mut self, delta_t: f32) {
        self.physics.tick(&mut self.space, delta_t)
    }
}

pub struct Space<T> {
    objects: HashMap<ObjectKey, T>,
}

impl<T> Space<T>
where
    T: Identity,
{
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add(&mut self, object: T) -> ObjectKey {
        let key = object.key();
        if self.objects.insert(key, object).is_some() {
            panic!("objects with same ObjectId({})", key.0)
        };
        key
    }

    pub fn destroy(&mut self, key: ObjectKey) {
        self.objects.remove(&key);
    }

    pub fn get(&self, key: ObjectKey) -> Option<&T> {
        self.objects.get(&key)
    }

    pub fn get_mut(&mut self, key: ObjectKey) -> Option<&mut T> {
        self.objects.get_mut(&key)
    }

    pub fn contains(&self, key: ObjectKey) -> bool {
        self.objects.contains_key(&key)
    }

    pub fn objects(&self) -> impl Iterator<Item = &T> {
        self.objects.values()
    }

    pub fn objects_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.objects.values_mut()
    }
}

impl<T> Space<T>
where
    T: Identity + Default,
{
    pub fn new_object(&mut self) -> ObjectKey {
        let obj = T::default();
        self.add(obj)
    }
}

impl<T> Space<T>
where
    T: Paint,
{
    pub fn paint(&self, viewport: &Viewport) -> Scene {
        let mut scene = Scene::new(viewport.width, viewport.height);
        for obj in self.objects.values() {
            if let Some(mut sprite) = obj.paint() {
                if viewport.overlaps(&sprite) {
                    sprite.shift_pos(viewport.pos * -1);
                    scene.add_sprite(sprite);
                }
            }
        }
        scene
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vector::v2::V2;

    #[test]
    fn test_viewport_relative_pos() {
        let viewport = Viewport::new(V2::new(-500, -500), 1000, 1000);
        let rp = viewport.relative_pos(V2::new(0.0, 0.0));
        assert_eq!(rp, V2::new(-500, -500));

        let rp = viewport.relative_pos(V2::new(1.0, 1.0));
        assert_eq!(rp, V2::new(500, 500));

        let rp = viewport.relative_pos(V2::new(0.0, 0.5));
        assert_eq!(rp, V2::new(-500, 0));
    }
}
