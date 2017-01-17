#[macro_use]
extern crate korome;
extern crate collider;

use korome::{Graphics, run_until_closed, FrameInfo, Drawer, VirtualKeyCode};
use collider::{Hitbox, Event};
use collider::geom::{Shape, Vec2, vec2};

const BALL_SPEED: f64 = 150.;
const PADDLE_SPEED: f64 = 400.;

#[macro_use]
mod macros;
mod obj;
use obj::*;
use obj::ObjectType::*;

fn main() {
    let g = Graphics::new("Arkanoid", 800, 600).unwrap();

    let mut obj_sys = ObjSys::new(&g);
    obj_sys.add_hb(Paddle, hitbox(0., -200., Shape::new_rect(vec2(100., 49.))));

    let mut ball = hitbox(0., 0., Shape::new_circle(16.));
    ball.vel.pos = vec2(BALL_SPEED, -BALL_SPEED);
    obj_sys.add_hb(Ball, ball.clone());

    run_until_closed(g, move |info: &FrameInfo, drawer: &mut Drawer| {
        let (w, h) = drawer.graphics.get_h_size();
        let wh = (w as f64, h as f64);
        let delta = if info.delta > 0.16666 {0.16666} else {info.delta as f64};

        obj_sys.add_time(delta, |obj_sys, e, id1, id2| {
            if let Event::Collide = e {
                if ObjectType::from_id(id1).is_paddle() && ObjectType::from_id(id2).is_ball() {
                    let paddle = obj_sys.get(id1);
                    let mut ball = obj_sys.get(id2);
                    ball.vel.pos.y *= -1.;
                    ball.vel.pos += paddle.vel.pos;
                    obj_sys.update(id2, ball);
                }
                if ObjectType::from_id(id1).is_ball() && ObjectType::from_id(id2).is_ball() {
                    let mut ball1 = obj_sys.get(id1);
                    let mut ball2 = obj_sys.get(id2);

                    let Vec2{x, y} = ball1.shape.normal_from(&ball2.shape).dir();
                    let theta = y.atan2(x);
                    let (sin, cos) = theta.sin_cos();
                    let v = vec2(cos, sin);

                    let x1 = ball1.vel.pos.rotate_rad(theta).x;
                    let x2 = ball2.vel.pos.rotate_rad(theta).x;

                    ball1.vel.pos += v * x1;
                    ball2.vel.pos += v * x2;

                    obj_sys.update(id1, ball1);
                    obj_sys.update(id2, ball2);
                }
            }
        });

        if info.get_key_events().contains(&(false, VirtualKeyCode::Space)) {
            obj_sys.add_hb(Ball, ball.clone());
        }
        if info.get_key_events().contains(&(false, VirtualKeyCode::G)) {
            println!("{:?}", obj_sys.count_balls());
        }

        obj_sys.update_all(&info, wh);

        drawer.clear(0., 0., 0.);
        obj_sys.draw_all(drawer);
        obj_sys.count_balls() != 0
    });
}

fn update_ball(hb: &mut Hitbox, _: &FrameInfo, (w, h): (f64, f64)) -> bool {
    if hb.shape.left() <= -w {
        hb.vel.pos.x =  hb.vel.pos.x.abs();
    } else if hb.shape.right() >= w{
        hb.vel.pos.x = -hb.vel.pos.x.abs();
    }
    if hb.shape.bottom() <= -h {
        false
    } else if hb.shape.top() >= h {
        hb.vel.pos.y = -BALL_SPEED;
        true
    } else { true }
}

fn update_player(hb: &mut Hitbox, info: &FrameInfo, (w, _h): (f64, f64)) {
    let mut vel = 0.;

    is_down!{info;
        Left, A => {
            vel += -PADDLE_SPEED;
        },
        Right, D => {
            vel += PADDLE_SPEED;
        }
    }

    hb.vel.pos = vec2(vel, 0.);
    if hb.shape.left() <= -w {
        hb.shape.pos.x = -w + hb.shape.dims().x/2.;
    } else if hb.shape.right() >= w {
        hb.shape.pos.x = w - hb.shape.dims().x/2.;
    }
}
