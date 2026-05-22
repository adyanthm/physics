use crate::{Vec2, dot};

pub const GRAVITY: f32 = 980.0;

pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub use_gravity: bool,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            pos: (0.0, 0.0),
            vel: (0.0, 0.0),
            use_gravity: true,
        }
    }
}

pub fn update(body: &mut Body, dt: f32) {
    if body.use_gravity {
        body.vel.1 += GRAVITY * dt;
    }
    body.pos.0 += body.vel.0 * dt;
    body.pos.1 += body.vel.1 * dt;
}

pub fn resolve_velocity(vel: &mut Vec2, normal: Vec2, restitution: f32) {
    let vn = dot(*vel, normal);
    if vn > 0.0 {
        return;
    }
    vel.0 -= (1.0 + restitution) * vn * normal.0;
    vel.1 -= (1.0 + restitution) * vn * normal.1;
}
