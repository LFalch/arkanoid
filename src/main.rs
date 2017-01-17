#[macro_use]
extern crate korome;
extern crate collider;

use korome::{Graphics, run_until_closed, FrameInfo, Drawer};
use collider::{Event};
use collider::geom::{Shape, vec2};

const BALL_SPEED: f64 = 150.;
const PADDLE_SPEED: f64 = 400.;

#[macro_use]
mod macros;
mod obj;
use obj::*;

fn main() {
    let g = Graphics::new("Arkanoid", 800, 600).unwrap();

    let mut obj_sys = ObjSys::new(&g);
    let paddle = obj_sys.add_hb(0, hitbox(0., -200., Shape::new_rect(vec2(100., 49.))));

    let mut ball = hitbox(0., 0., Shape::new_circle(16.));
    ball.vel.pos = vec2(BALL_SPEED, -BALL_SPEED);
    let ball = obj_sys.add_hb(5, ball);

    run_until_closed(g, move |info: &FrameInfo, drawer: &mut Drawer| {
        let (w, h) = drawer.graphics.get_h_size();
        let (w, h) = (w as f64, h as f64);
        let delta = if info.delta > 0.16666 {0.16666} else {info.delta as f64};

        let events = obj_sys.add_time(delta);

        for (e, _id1, _id2) in events {
            if let Event::Collide = e {
                let paddle = obj_sys.get(0);
                let mut ball = obj_sys.get(5);
                ball.vel.pos.y *= -1.;
                ball.vel.pos += paddle.vel.pos;
                obj_sys.update(5, ball);
            }
        }

        let mut vel = 0.;

        is_down!{info;
            Left, A => {
                vel += -PADDLE_SPEED;
            },
            Right, D => {
                vel += PADDLE_SPEED;
            }
        }
        let mut hb = obj_sys.get(0);
        hb.vel.pos = vec2(vel, 0.);
        if hb.shape.left() <= -w {
            hb.shape.pos.x = -w + hb.shape.dims().x/2.;
        } else if hb.shape.right() >= w {
            hb.shape.pos.x = w - hb.shape.dims().x/2.;
        }
        obj_sys.update(0, hb);

        let mut hb = obj_sys.get(5);
        if hb.shape.left() <= -w {
            hb.vel.pos.x =  hb.vel.pos.x.abs();
        } else if hb.shape.right() >= w{
            hb.vel.pos.x = -hb.vel.pos.x.abs();
        }
        if hb.shape.bottom() <= -h {
            return false;
        } else if hb.shape.top() >= h {
            hb.vel.pos.y = -BALL_SPEED;
        }
        obj_sys.update(5, hb);


        drawer.clear(0., 0., 0.);
        obj_sys.draw(paddle, drawer);
        obj_sys.draw(ball, drawer);
        true
    });
}
