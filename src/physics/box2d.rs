use std::collections::{HashMap, HashSet};

use crate::{
    num::Float,
    pair::Pair,
    vector::vec2d::{Axis, Vec2d as V},
};

pub use super::identity::ObjectId;
use super::{
    identity::{Identity, IdentityKey, ObjectKey},
    universe::{Physics, Space, Universe},
};

pub struct Mov<T> {
    pub pos: V<T>,
    pub vel: V<T>,
    pub acc: V<T>,
}

impl<T> Mov<T>
where
    T: Float,
{
    pub fn update(&mut self, delta_t: f32) {
        let delta_t: T = T::from_f32(delta_t).unwrap();
        self.vel = self.vel + self.acc * delta_t;
        self.pos = self.pos + self.vel * delta_t;
    }
}

pub enum Mass<T> {
    Infinite,
    Density(T),
    Fixed(T),
}

pub struct HitBox<T> {
    pub width: usize,
    pub height: usize,
    pub mov: Mov<T>,
    pub mass: Mass<T>,
}

impl<T> HitBox<T>
where
    T: Float,
{
    pub fn bounds(&self) -> (T, T, T, T) {
        let x0 = self.mov.pos.x;
        let x1 = x0 + T::from_usize(self.width).unwrap();
        let y0 = self.mov.pos.y;
        let y1 = y0 + T::from_usize(self.height).unwrap();
        (x0, x1, y0, y1)
    }

    pub fn dimension(&self, axis: Axis) -> usize {
        match axis {
            Axis::X => self.width,
            Axis::Y => self.height,
        }
    }

    pub fn mass(&self) -> Option<T> {
        match self.mass {
            Mass::Infinite => None,
            Mass::Fixed(m) => Some(m),
            Mass::Density(m) => {
                let area: T = T::from_usize(self.width * self.height).unwrap();
                Some(area * m)
            }
        }
    }
}

pub trait Object<T>: Identity + IdentityKey {
    fn hitbox(&self) -> &HitBox<T>;
    fn hitbox_mut(&mut self) -> &mut HitBox<T>;
}

// Inelastic collision
// K = Ma * Ua + Mb * Ub
// Va = (K + Mb * COR *(Ub - Ua)) / (Ma + Mb)
// Vb = (K + Ma * COR * (Ua - Ub)) / (Ma + Mb)
fn collide_m_to_m<T>(m_a: T, u_a: T, m_b: T, u_b: T, cor: T) -> (T, T)
where
    T: Float,
{
    let k = m_a * u_a + m_b * u_b;
    let v_a = (k + m_b * cor * (u_b - u_a)) / (m_a + m_b);
    let v_b = (k + m_a * cor * (u_a - u_b)) / (m_a + m_b);
    (v_a, v_b)
}

// Inelastic collision for Limit Mb -> inf
// Va = Ub + COR * (Ub - Ua)
// Vb = Ub
fn collide_m_to_inf<T>(u_a: T, u_b: T, cor: T) -> (T, T)
where
    T: Float,
{
    let v_a = u_b + cor * (u_b - u_a);
    let v_b = u_b;
    (v_a, v_b)
}

// Inelastic collision for Limit Ma -> inf, Mb -> inf
// K = Ua + Ub
// Va = (K + COR * (Ub - Ua)) / 2
// Vb = (K + COR * (Ua - Ub)) / 2
fn collide_inf_to_inf<T>(u_a: T, u_b: T, cor: T) -> (T, T)
where
    T: Float,
{
    let k = u_a + u_b;
    let two = T::from_f32(2.0).unwrap();
    let v_a = (k + cor * (u_b - u_a)) / two;
    let v_b = (k + cor * (u_a - u_b)) / two;
    (v_a, v_b)
}

fn collide<O, T>(a: &mut O, b: &mut O, axis: Axis, cor: T)
where
    O: Object<T>,
    T: Float,
{
    let hb_a = a.hitbox_mut();
    let m_a = hb_a.mass();
    let u_a = *hb_a.mov.vel.ax(axis);

    let hb_b = b.hitbox_mut();
    let m_b = hb_b.mass();
    let u_b = *hb_b.mov.vel.ax(axis);

    let (v_a, v_b) = match (m_a, m_b) {
        (Some(m_a), Some(m_b)) => collide_m_to_m(m_a, u_a, m_b, u_b, cor),
        (None, None) => collide_inf_to_inf(u_a, u_b, cor),
        (Some(_), None) => collide_m_to_inf(u_a, u_b, cor),
        (None, Some(_)) => {
            let (v_b, v_a) = collide_m_to_inf(u_b, u_a, cor);
            (v_a, v_b)
        }
    };
    *a.hitbox_mut().mov.vel.ax_mut(axis) = v_a;
    *b.hitbox_mut().mov.vel.ax_mut(axis) = v_b;
}

pub struct Collider<T> {
    cor: T,
    last_colliding_x: HashSet<Pair<ObjectKey>>,
    last_colliding_y: HashSet<Pair<ObjectKey>>,
}

impl<T> Collider<T>
where
    T: Float,
{
    pub fn new(cor: T) -> Self {
        Self {
            cor,
            last_colliding_x: HashSet::new(),
            last_colliding_y: HashSet::new(),
        }
    }

    fn unclip<O>(&self, a: &mut O, b: &mut O) -> Axis
    where
        O: Object<T>,
    {
        let (a_x0, a_x1, a_y0, a_y1) = a.hitbox().bounds();
        let (b_x0, b_x1, b_y0, b_y1) = b.hitbox().bounds();

        // Find the direction that requires the minimum amount of
        // movement to unclip the objects
        let potential_a_unclips = [
            V::new(b_x1 - a_x0, T::zero()),
            V::new(b_x0 - a_x1, T::zero()),
            V::new(T::zero(), b_y1 - a_y0),
            V::new(T::zero(), b_y0 - a_y1),
        ];
        let a_unclip = potential_a_unclips
            .iter()
            .min_by(|a, b| a.mag().partial_cmp(&b.mag()).unwrap())
            .unwrap();

        let unclip_axis = if a_unclip.x == T::zero() {
            Axis::Y
        } else {
            Axis::X
        };

        // Assign unclip movement to a and b based on their mass
        let (a_unclip, b_unclip) = match (a.hitbox().mass(), b.hitbox().mass()) {
            (None, None) => {
                let a_unclip = *a_unclip * T::from_f32(0.5).unwrap();
                let b_unclip = a_unclip * -T::one();
                (a_unclip, b_unclip)
            }
            (Some(_), None) => (*a_unclip, V::zero()),
            (None, Some(_)) => (V::zero(), *a_unclip * -T::one()),
            (Some(ma), Some(mb)) => {
                let ma_unclip = *a_unclip * (mb / (ma + mb));
                let mb_unclip = *a_unclip * (ma / (ma + mb)) * -T::one();
                (ma_unclip, mb_unclip)
            }
        };
        let a_new_pos = a.hitbox().mov.pos + a_unclip;
        let b_new_pos = b.hitbox().mov.pos + b_unclip;
        a.hitbox_mut().mov.pos = a_new_pos;
        b.hitbox_mut().mov.pos = b_new_pos;

        unclip_axis
    }

    pub fn collide<O>(&mut self, objects: &mut [&mut O])
    where
        O: Object<T>,
        T: Float,
    {
        let idx_by_id: HashMap<ObjectKey, usize> = objects
            .iter()
            .enumerate()
            .map(|(idx, obj)| (obj.key(), idx))
            .collect();

        let scanlist_x = scan_axis(objects, Axis::X);
        let colliding_x = find_axis_collisions(&scanlist_x);
        if colliding_x.is_empty() {
            return;
        }
        let scanlist_y = scan_axis(objects, Axis::Y);
        let colliding_y = find_axis_collisions(&scanlist_y);
        let mut colliding_xy: HashSet<Pair<ObjectKey>> = HashSet::new();
        for k in colliding_x.iter() {
            if colliding_y.contains(k) {
                colliding_xy.insert(*k);
            }
        }
        for pair in colliding_xy.iter() {
            let pairt = pair.tuple();
            let left_idx = idx_by_id.get(pairt.0).unwrap();
            let right_idx = idx_by_id.get(pairt.1).unwrap();
            assert_ne!(left_idx, right_idx, "same object!?");

            let last_x_collided = self.last_colliding_x.contains(pair);
            let last_y_collided = self.last_colliding_y.contains(pair);

            let collide_axes = match (last_x_collided, last_y_collided) {
                (false, true) => vec![Axis::X],
                (true, false) => vec![Axis::Y],
                (false, false) => vec![Axis::X, Axis::Y],
                (true, true) => {
                    let axis: Axis;
                    unsafe {
                        let obj1: *mut O = objects[*left_idx] as *mut O;
                        let obj2: *mut O = objects[*right_idx] as *mut O;
                        axis = self.unclip(&mut *obj1, &mut *obj2);
                    }
                    vec![axis]
                }
            };
            unsafe {
                let obj1: *mut O = objects[*left_idx] as *mut O;
                let obj2: *mut O = objects[*right_idx] as *mut O;
                for axis in collide_axes {
                    collide(&mut *obj1, &mut *obj2, axis, self.cor);
                }
            }
        }
        self.last_colliding_x = colliding_x;
        self.last_colliding_y = colliding_y;
    }
}

fn scan_axis<O, T>(objects: &[&mut O], axis: Axis) -> Vec<(T, ObjectKey, bool)>
where
    T: Float,
    O: Object<T>,
{
    let mut coords: Vec<(T, ObjectKey, bool)> = Vec::with_capacity(2 * objects.len());
    for object in objects.iter() {
        let hb = object.hitbox();
        let c = *hb.mov.pos.ax(axis);
        let w = T::from_usize(hb.dimension(axis)).unwrap();
        coords.push((c, object.key(), true));
        coords.push((c + w, object.key(), false));
    }
    coords.sort_by(|(c1, _, _), (c2, _, _)| c1.partial_cmp(c2).unwrap());
    coords
}

fn find_axis_collisions<T>(scanlist: &[(T, ObjectKey, bool)]) -> HashSet<Pair<ObjectKey>>
where
    T: Float,
{
    if scanlist.is_empty() {
        return HashSet::new();
    }
    let mut colliding: HashSet<Pair<ObjectKey>> = HashSet::new();
    let mut intersecting: HashSet<ObjectKey> = HashSet::new();
    for (_, id, is_in) in scanlist {
        if *is_in {
            for other_id in intersecting.iter() {
                let p: Pair<ObjectKey> = Pair::new(*id, *other_id);
                colliding.insert(p);
            }
            intersecting.insert(*id);
        } else {
            intersecting.remove(id);
        }
    }
    colliding
}

pub struct Box2DPhysics<T> {
    collider: Collider<T>,
}

impl<T> Box2DPhysics<T>
where
    T: Float,
{
    pub fn new(cor: T) -> Self {
        Self {
            collider: Collider::new(cor),
        }
    }
}

impl<T, O> Physics<O> for Box2DPhysics<T>
where
    T: Float,
    O: Object<T>,
{
    fn tick(&mut self, space: &mut Space<O>, delta_t: f32) {
        let mut objects: Vec<&mut O> = space.objects_mut().collect();
        self.collider.collide::<O>(&mut objects);
        for obj in objects {
            obj.hitbox_mut().mov.update(delta_t);
        }
    }
}

pub fn box2d_universe<T, O>(cor: T) -> Universe<Box2DPhysics<T>, O>
where
    T: Float,
    O: Object<T>,
{
    Universe::new(Box2DPhysics::new(cor))
}
