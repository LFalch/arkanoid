use korome::{Graphics, Texture, Drawer, FrameInfo};
use collider::{Collider, Hitbox, HitboxId, Event};
use collider::geom::{PlacedShape, Shape, Vec2, vec2};
use collider::inter::{Interactivity, Group};

use std::collections::BTreeSet;

pub struct ObjSys {
    collider: Collider<ObjectType>,
    time: f64,
    objs: BTreeSet<HitboxId>,
    pub tb: TextureBase
}

texturebase!{TextureBase;
    paddle "paddle",
    ball "newred",
}

impl ObjSys {
    pub fn new(g: &Graphics) -> Self {
        ObjSys {
            collider: Collider::new(50., 0.01),
            time: 0.,
            objs: BTreeSet::new(),
            tb: TextureBase::new(g)
        }
    }
    fn free_id_by_type(&self, ty: ObjectType) -> HitboxId {
        (ty as u32 as HitboxId..).filter(|i| !self.objs.contains(i)).next().unwrap()
    }
    pub fn add_hb(&mut self, group: ObjectType, hb: Hitbox) {
        let id = self.free_id_by_type(group);
        self.collider.add_hitbox_with_interactivity(id, hb, ObjectType::from_id(id));
        self.objs.insert(id);
    }
    fn draw(&self, id: HitboxId, d: &mut Drawer) {
        let Vec2{x, y} = self.get(id).shape.pos;
        match ObjectType::from_id(id) {
            Paddle => &self.tb.paddle,
            Ball => &self.tb.ball,
            _ => unimplemented!()
        }.drawer()
         .pos((x as f32, y as f32))
         .draw(d)
    }
    pub fn draw_all(&self, d: &mut Drawer) {
        for &id in self.objs.iter() {
            self.draw(id, d);
        }
    }
    pub fn update_all(&mut self, info: &FrameInfo, wh: (f64, f64)) {
        let mut dead_balls = Vec::new();
        for &id in &self.objs {
            let ty = ObjectType::from_id(id);
            let mut hb = self.get(id);
            match ty {
                Paddle => {
                    super::update_player(&mut hb, info, wh);
                }
                Ball => {
                    if !super::update_ball(&mut hb, info, wh) {
                        dead_balls.push(id);
                    }
                }
                _ => unimplemented!()
            }
            self.collider.update_hitbox(id, hb);
        }
        for id in dead_balls {
            self.objs.remove(&id);
            self.collider.remove_hitbox(id);
        }
    }
    pub fn add_time<F: FnMut(&mut Self, Event, HitboxId, HitboxId)>(&mut self, delta: f64, mut f: F) {
        self.time += delta;

        let mut next_time = self.collider.next_time();
        while self.time >= next_time {
            self.collider.set_time(next_time);
            while let Some((e, id1, id2)) = self.collider.next() {
                f(self, e, id1, id2);
            }
            next_time = self.collider.next_time();
        }
        self.collider.set_time(self.time);
    }
    #[inline]
    pub fn get(&self, id: HitboxId) -> Hitbox {
        self.collider.get_hitbox(id)
    }
    #[inline]
    pub fn update(&mut self, id: HitboxId, new: Hitbox) {
        self.collider.update_hitbox(id, new)
    }
    pub fn count_balls(&self) -> usize {
        self.objs.iter().filter(|&&i| ObjectType::from_id(i) == Ball).count()
    }
}

pub fn hitbox(x: f64, y: f64, shape: Shape) -> Hitbox {
    Hitbox::new(PlacedShape::new(vec2(x, y), shape))
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ObjectType {
    Paddle = 0,
    Ball = 1,
    Brick = 1000,
}
use self::ObjectType::*;

pub const PADDLE: u32 = Paddle as u32;
pub const BALL: u32 = Ball as u32;
pub const BRICK: u32 = Brick as u32;

impl ObjectType {
    pub fn from_id(id: HitboxId) -> Self {
        if id >= BRICK as HitboxId {
            Brick
        } else if id >= BALL as HitboxId {
            Ball
        } else if id >= BRICK as HitboxId {
            Brick
        } else {
            Paddle
        }
    }
    pub fn is_paddle(&self) -> bool {
        if let Paddle = *self {
            true
        } else {
            false
        }
    }
    pub fn is_ball(&self) -> bool {
        if let Ball = *self {
            true
        } else {
            false
        }
    }
}

impl Interactivity for ObjectType {
    fn can_interact(&self, other: &Self) -> bool {
        self.interact_groups().contains(&other.group().unwrap())
    }
    fn group(&self) -> Option<Group> {
        Some(*self as u32)
    }
    fn interact_groups(&self) -> &'static [Group] {
        const BALL_GRP: &'static [Group] = &[PADDLE, BRICK, BALL];
        const NORMAL_GRP: &'static [Group] = &[BALL];

        match *self {
            Ball => BALL_GRP,
            _ => NORMAL_GRP
        }
    }
}
