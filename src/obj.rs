use korome::{Graphics, Texture, Drawer};
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
    paddle "paddle_gun",
    ball "red",
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
    pub fn add_hb(&mut self, id: HitboxId, hb: Hitbox) -> HitboxId {
        self.collider.add_hitbox_with_interactivity(id, hb, ObjectType::from_id(id));
        id
    }
    pub fn draw(&self, id: HitboxId, d: &mut Drawer) {
        let Vec2{x, y} = self.get(id).shape.pos;
        match ObjectType::from_id(id) {
            Paddle => &self.tb.paddle,
            Ball => &self.tb.ball,
            _ => unimplemented!()
        }.drawer()
         .pos((x as f32, y as f32))
         .draw(d)
    }
    pub fn add_time(&mut self, delta: f64) -> Vec<(Event, HitboxId, HitboxId)> {
        self.time += delta;

        let mut next_time = self.collider.next_time();
        let mut events = Vec::new();
        while self.time >= next_time {
            self.collider.set_time(next_time);
            while let Some(event) = self.collider.next() {
                events.push(event);
            }
            next_time = self.collider.next_time();
        }
        self.collider.set_time(self.time);
        events
    }
    #[inline]
    pub fn get(&self, id: HitboxId) -> Hitbox {
        self.collider.get_hitbox(id)
    }
    #[inline]
    pub fn update(&mut self, id: HitboxId, new: Hitbox) {
        self.collider.update_hitbox(id, new)
    }
}

pub fn hitbox(x: f64, y: f64, shape: Shape) -> Hitbox {
    Hitbox::new(PlacedShape::new(vec2(x, y), shape))
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ObjectType {
    Paddle = 0,
    Bottom = 1,
    Ball = 2,
    Brick = 1000,
}
use self::ObjectType::*;

pub const PADDLE: u32 = Paddle as u32;
pub const BOTTOM: u32 = Bottom as u32;
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
        } else if id >= BOTTOM as HitboxId {
            Bottom
        } else {
            Paddle
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
        const BALL_GRP: &'static [Group] = &[PADDLE, BOTTOM, BRICK, BALL];
        const NORMAL_GRP: &'static [Group] = &[BALL];

        match *self {
            Ball => BALL_GRP,
            _ => NORMAL_GRP
        }
    }
}
