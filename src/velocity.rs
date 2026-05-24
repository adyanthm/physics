use crate::Vec2;

pub const GRAVITY: f32 = 980.0;

pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub use_gravity: bool,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            use_gravity: true,
        }
    }
}

pub fn update(body: &mut Body, dt: f32) {
    if body.use_gravity {
        body.vel.y += GRAVITY * dt;
    }
    body.pos += body.vel * dt;
}

pub fn resolve_velocity(vel: &mut Vec2, normal: Vec2, restitution: f32) {
    let vn = vel.dot(normal);
    if vn > 0.0 {
        return;
    }
    *vel -= normal * ((1.0 + restitution) * vn);
}
