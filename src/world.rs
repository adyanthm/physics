use crate::body::{BodyType, RigidBody};
use crate::sat::advanced_collision;
use crate::{Vec2, resolve_collision};

pub const GRAVITY: f32 = 980.0;
pub type BodyId = usize;

#[derive(Clone)]
pub struct ContactEvent {
    pub body_a: BodyId,
    pub body_b: BodyId,
    pub normal: Vec2,
    pub depth: f32,
    pub contacts: Vec<Vec2>,
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
                return c.normal.y < -0.5;
            } else if c.body_b == id {
                return c.normal.y > 0.5;
            }
        }
        false
    }

    pub fn step(&mut self, dt: f32) {
        let gravity_dt = self.gravity * dt;
        for body in &mut self.bodies {
            body.bias_vel = Vec2::ZERO;
            body.bias_angular_vel = 0.0;
            if let BodyType::Dynamic = body.body_type {
                if body.angular_vel.abs() < 0.05 {
                    body.angular_vel = 0.0;
                }
                if body.vel.magnitude_sq() < 10.0 {
                    body.vel = Vec2::ZERO;
                }


                if body.vel.magnitude_sq() < 25.0 && body.angular_vel.abs() < 0.5 {
                    body.sleep_timer += dt;
                    if body.sleep_timer > 0.5 {
                        body.awake = false;
                        body.vel = Vec2::ZERO;
                        body.angular_vel = 0.0;
                    }
                } else {
                    body.sleep_timer = 0.0;
                    body.awake = true;
                }

                if body.awake && body.use_gravity {
                    body.vel.y += gravity_dt;
                }
            }
        }

        self.contacts.clear();
        let len = self.bodies.len();

        for i in 0..len {
            for j in (i + 1)..len {
                let active_i =
                    self.bodies[i].body_type == BodyType::Dynamic && self.bodies[i].awake;
                let active_j =
                    self.bodies[j].body_type == BodyType::Dynamic && self.bodies[j].awake;

                if !active_i && !active_j {
                    continue;
                }

                let verts_i = self.bodies[i].world_vertices();
                let verts_j = self.bodies[j].world_vertices();

                let speed_sq_i = self.bodies[i].vel.magnitude_sq();
                let speed_sq_j = self.bodies[j].vel.magnitude_sq();

                let ccd_threshold_sq = 575.0 * 575.0;
                let is_fast_collision =
                    speed_sq_i > ccd_threshold_sq || speed_sq_j > ccd_threshold_sq;

                if is_fast_collision {
                    if let Some(ccd) = crate::ccd::fast_poly_collide(
                        &verts_i,
                        &verts_j,
                        self.bodies[i].vel,
                        self.bodies[j].vel,
                        dt,
                    ) {
                        let rv = self.bodies[j].vel - self.bodies[i].vel;
                        let vel_along_normal = rv.dot(ccd.normal);

                        if vel_along_normal < 0.0 {
                            let e = self.bodies[i].restitution.min(self.bodies[j].restitution);
                            let inv_mass_sum = self.bodies[i].inv_mass + self.bodies[j].inv_mass;

                            if inv_mass_sum > 0.0 {
                                let j_impulse = -(1.0 + e) * vel_along_normal / inv_mass_sum;
                                let impulse = ccd.normal * j_impulse;

                                let inv_mass_i = self.bodies[i].inv_mass;
                                let inv_mass_j = self.bodies[j].inv_mass;
                                self.bodies[i].vel -= impulse * inv_mass_i;
                                self.bodies[j].vel += impulse * inv_mass_j;
                            }
                        }
                    }
                }

                if let Some(c) = advanced_collision(&verts_i, &verts_j) {
                    if self.bodies[i].body_type == BodyType::Dynamic {
                        self.bodies[i].awake = true;
                        self.bodies[i].sleep_timer = 0.0;
                    }
                    if self.bodies[j].body_type == BodyType::Dynamic {
                        self.bodies[j].awake = true;
                        self.bodies[j].sleep_timer = 0.0;
                    }

                    self.contacts.push(ContactEvent {
                        body_a: i,
                        body_b: j,
                        normal: c.normal,
                        depth: c.depth,
                        contacts: c.contacts,
                    });
                }
            }
        }

        let iterations = 50;
        for _ in 0..iterations {
            for ContactEvent {
                body_a: i,
                body_b: j,
                normal,
                depth,
                contacts,
            } in self.contacts.clone()
            {
                let (left, right) = self.bodies.split_at_mut(j);
                let body_i = &mut left[i];
                let body_j = &mut right[0];

                if body_i.body_type == BodyType::Static && body_j.body_type == BodyType::Static {
                    continue;
                }

                for &contact in &contacts {
                    resolve_collision(body_i, body_j, -normal, contact, depth, dt);
                }
            }
        }

        for body in &mut self.bodies {
            if let BodyType::Dynamic = body.body_type {
                if body.awake {
                    body.pos += (body.vel + body.bias_vel) * dt;
                    body.angle += (body.angular_vel + body.bias_angular_vel) * dt;
                    body.angular_vel *= 0.99;
                }
            }
        }
    }
}
