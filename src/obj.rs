use korome::{Graphics, Texture, Drawer};
use collider::{Collider, Hitbox, HitboxId};
use collider::geom::{PlacedShape, Shape, Vec2, vec2};
use collider::inter::{Interactivity, Group};

pub struct Object {
    pub id: HitboxId,
    tex: Texture
}

impl Object {
    pub fn new(g: &Graphics, tex: &str, id: HitboxId) -> Self{
        Object {
            id: id,
            tex: Texture::from_file(&g, tex).unwrap()
        }
    }
    pub fn draw(&self, c: &Collider<ObjectType>, d: &mut Drawer) {
        let Vec2{x, y} = c.get_hitbox(self.id).shape.pos;
        self.tex.drawer()
            .pos((x as f32, y as f32))
            .draw(d)
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
    Brick = 2,
    BoundaryWall = 3,
}

use self::ObjectType::*;

impl Interactivity for ObjectType {
    fn can_interact(&self, other: &Self) -> bool {
        self.interact_groups().contains(&other.group().unwrap())
    }
    fn group(&self) -> Option<Group> {
        Some(*self as u32)
    }
    fn interact_groups(&self) -> &'static [Group] {
        const BALL_INTERACT_GROUPS: &'static [Group] = &[Paddle as u32,
            Brick as u32, BoundaryWall as u32];
        const NORMAL_INTERACT_GROUPS: &'static [Group] = &[Ball as u32];

        match *self {
            Ball => BALL_INTERACT_GROUPS,
            _ => NORMAL_INTERACT_GROUPS
        }
    }
}
