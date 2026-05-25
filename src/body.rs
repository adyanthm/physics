use crate::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub enum BodyType {
    Dynamic,
    Static,
}

pub struct RigidBody {
    pub pos: Vec2,
    pub vel: Vec2,
    pub body_type: BodyType,
    pub use_gravity: bool,
    pub restitution: f32,
    pub vertices: Vec<Vec2>,
}

impl RigidBody {
    pub fn new_dynamic(pos: Vec2, vertices: Vec<Vec2>) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            body_type: BodyType::Dynamic,
            use_gravity: true,
            restitution: 0.0,
            vertices,
        }
    }
    pub fn new_static(pos: Vec2, vertices: Vec<Vec2>) -> Self {
        Self {
            pos,
            vel: Vec2::ZERO,
            body_type: BodyType::Static,
            use_gravity: true,
            restitution: 0.0,
            vertices,
        }
    }

    pub fn world_vertices(&self) -> Vec<Vec2> {
        self.vertices.iter().map(|&v| v + self.pos).collect()
    }
}

pub fn rect_vertices(w: f32, h: f32) -> Vec<Vec2> {
    vec![
        Vec2::ZERO,
        Vec2::new(w, 0.0),
        Vec2::new(w, h),
        Vec2::new(0.0, h),
    ]
}
