#[macro_use]
extern crate korome;
extern crate collider;

use korome::{Graphics, Texture, run_until_closed, FrameInfo, Drawer};
use collider::{Collider, Event, Hitbox, HitboxId};
use collider::geom::{PlacedShape, Shape, Vec2, vec2};

struct Object {
    id: HitboxId,
    tex: Texture
}

impl Object {
    fn new(g: &Graphics, tex: &str, id: HitboxId) -> Self{
        Object {
            id: id,
            tex: Texture::from_file(&g, tex).unwrap()
        }
    }
    fn draw(&self, c: &Collider, d: &mut Drawer) {
        let Vec2{x, y} = c.get_hitbox(self.id).shape.pos;
        self.tex.drawer()
            .pos((x as f32, y as f32))
            .draw(d)
    }
}

const BALL_SPEED: f64 = 150.;
const PADDLE_SPEED: f64 = 400.;

fn hitbox(x: f64, y: f64, shape: Shape) -> Hitbox {
    Hitbox::new(PlacedShape::new(vec2(x, y), shape))
}

fn main() {
    let g = Graphics::new("Arkanoid", 800, 600).unwrap();

    let mut collider = Collider::new(50., 0.01);
    collider.add_hitbox(0, hitbox(0., -200., Shape::new_rect(vec2(100., 49.))));
    let paddle = Object::new(&g, "textures/paddle_gun.png", 0);

    let mut ball = hitbox(0., 0., Shape::new_circle(16.));
    ball.vel.pos = vec2(BALL_SPEED, -BALL_SPEED);
    collider.add_hitbox(1, ball);
    let ball = Object::new(&g, "textures/red.png", 1);

    let mut time = 0.;

    run_until_closed(g, |info: &FrameInfo, drawer: &mut Drawer| {
        let (w, h) = drawer.graphics.get_h_size();
        let (w, h) = (w as f64, h as f64);
        let delta = if info.delta > 0.16666 {0.16666} else {info.delta as f64};
        time += delta;

        let mut next_time = collider.next_time();
        while time >= next_time {
            collider.set_time(next_time);
            while let Some((e, _id1, _id2)) = collider.next() {
                if let Event::Collide = e {
                    let mut ball = collider.get_hitbox(1);
                    ball.vel.pos.y *= -1.;
                    collider.update_hitbox(1, ball);
                }
            }
            next_time = collider.next_time();
        }
        collider.set_time(time);

        let mut vel = 0.;

        is_down!{info;
            Left, A => {
                vel += -PADDLE_SPEED;
            },
            Right, D => {
                vel += PADDLE_SPEED;
            }
        }
        let mut hb = collider.get_hitbox(0);
        hb.vel.pos = vec2(vel, 0.);
        if hb.shape.left() <= -w {
            hb.shape.pos.x = -w + hb.shape.dims().x/2.;
        } else if hb.shape.right() >= w {
            hb.shape.pos.x = w - hb.shape.dims().x/2.;
        }
        collider.update_hitbox(0, hb);

        let mut hb = collider.get_hitbox(1);
        if hb.shape.left() <= -w {
            hb.vel.pos.x = BALL_SPEED;
        } else if hb.shape.right() >= w{
            hb.vel.pos.x = -BALL_SPEED;
        }
        if hb.shape.bottom() <= -h {
            hb.vel.pos.y = BALL_SPEED;
        } else if hb.shape.top() >= h {
            hb.vel.pos.y = -BALL_SPEED;
        }
        collider.update_hitbox(1, hb);


        drawer.clear(0., 0., 0.);
        paddle.draw(&collider, drawer);
        ball.draw(&collider, drawer);
    });
    println!("Ended at time: {}s", time);
}
