use crate::body::{BodyType, RigidBody};
use crate::sat::{advanced_collision, resolve_pos, resolve_pos_static};
use crate::{Vec2, resolve_velocity};

pub const GRAVITY: f32 = 980.0;
pub type BodyId = usize;

#[derive(Clone, Copy)]
pub struct ContactEvent {
    pub body_a: BodyId,
    pub body_b: BodyId,
    pub normal: Vec2,
    pub depth: f32,
}

pub struct PhysicsWorld {
    pub bodies: Vec<RigidBody>,
    pub contacts: Vec<ContactEvent>,
    pub gravity: f32,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self {
            bodies: Vec::new(),
            contacts: Vec::new(),
            gravity: GRAVITY,
        }
    }
}
impl PhysicsWorld {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_body(&mut self, body: RigidBody) -> BodyId {
        self.bodies.push(body);
        self.bodies.len() - 1
    }
    pub fn is_grounded(&self, id: BodyId) -> bool {
        for c in &self.contacts {
            if c.body_a == id {
                return c.normal.y < 0.5;
            } else if c.body_b == id {
                return c.normal.y > 0.5;
            }
        }
        false
    }
    pub fn step(&mut self, dt: f32) {
        let gravity_dt = self.gravity * dt;
        for body in &mut self.bodies {
            if let BodyType::Dynamic = body.body_type {
                if body.use_gravity {
                    body.vel.y += gravity_dt;
                }
                body.pos += body.vel * dt;
            }
        }

        self.contacts.clear();
        let len = self.bodies.len();

        for i in 0..len {
            for j in (i + 1)..len {
                let verts_i = self.bodies[i].world_vertices();
                let verts_j = self.bodies[j].world_vertices();

                if let Some(c) = advanced_collision(&verts_i, &verts_j) {
                    self.contacts.push(ContactEvent {
                        body_a: i,
                        body_b: j,
                        normal: c.normal,
                        depth: c.depth,
                    });
                }
            }
        }

        for &ContactEvent {
            body_a: i,
            body_b: j,
            normal,
            depth,
        } in &self.contacts
        {
            let (left, right) = self.bodies.split_at_mut(j);
            let body_i = &mut left[i];
            let body_j = &mut right[0];

            match (&body_i.body_type, &body_j.body_type) {
                (BodyType::Dynamic, BodyType::Static) => {
                    resolve_pos_static(&mut body_i.pos, normal, depth);
                    resolve_velocity(&mut body_i.vel, normal, body_i.restitution);
                }
                (BodyType::Static, BodyType::Dynamic) => {
                    resolve_pos_static(&mut body_j.pos, -normal, depth);
                    resolve_velocity(&mut body_j.vel, -normal, body_j.restitution);
                }
                (BodyType::Dynamic, BodyType::Dynamic) => {
                    resolve_pos(&mut body_i.pos, &mut body_j.pos, normal, depth);
                    resolve_velocity(&mut body_i.vel, normal, body_i.restitution);
                    resolve_velocity(&mut body_j.vel, -normal, body_j.restitution);
                }
                _ => {}
            }
        }
    }
}
