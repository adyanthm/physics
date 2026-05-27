use crate::Vec2;
use crate::body::RigidBody;

pub fn resolve_collision(
    body_a: &mut RigidBody,
    body_b: &mut RigidBody,
    normal: Vec2,
    contact: Vec2,
    depth: f32,
    dt: f32,
) {
    let ra = contact - body_a.pos;
    let rb = contact - body_b.pos;

    let va = body_a.vel + ra.perp() * body_a.angular_vel;
    let vb = body_b.vel + rb.perp() * body_b.angular_vel;
    let rv = vb - va;

    let vel_along_normal = rv.dot(normal);

    let ra_cross_n = ra.cross(normal);
    let rb_cross_n = rb.cross(normal);

    let inv_mass_sum = body_a.inv_mass
        + body_b.inv_mass
        + ra_cross_n * ra_cross_n * body_a.inv_inertia
        + rb_cross_n * rb_cross_n * body_b.inv_inertia;

    if vel_along_normal <= 0.0 {
        let mut e = body_a.restitution.min(body_b.restitution);

        if vel_along_normal > -30.0 {
            e = 0.0;
        }

        if inv_mass_sum > 0.0 {

            let j = -(1.0 + e) * vel_along_normal / inv_mass_sum;
            let impulse = normal * j;

            body_a.vel -= impulse * body_a.inv_mass;
            body_a.angular_vel -= ra.cross(impulse) * body_a.inv_inertia;

            body_b.vel += impulse * body_b.inv_mass;
            body_b.angular_vel += rb.cross(impulse) * body_b.inv_inertia;
        }
    }

    let va = body_a.vel + ra.perp() * body_a.angular_vel;
    let vb = body_b.vel + rb.perp() * body_b.angular_vel;
    let rv = vb - va;

    let tangent = rv - normal * rv.dot(normal);
    let tangent_len = tangent.magnitude();
    if tangent_len > f32::EPSILON {
        let t = tangent / tangent_len;
        let ra_cross_t = ra.cross(t);
        let rb_cross_t = rb.cross(t);

        let inv_mass_sum_t = body_a.inv_mass
            + body_b.inv_mass
            + ra_cross_t * ra_cross_t * body_a.inv_inertia
            + rb_cross_t * rb_cross_t * body_b.inv_inertia;

        if inv_mass_sum_t > 0.0 {
            let jt = -rv.dot(t) / inv_mass_sum_t;
            let friction = (body_a.friction * body_b.friction).sqrt();

            let j_approx = (-vel_along_normal / inv_mass_sum).max(0.0);

            let friction_impulse = if jt.abs() < j_approx * friction {
                t * jt
            } else {
                t * (j_approx * friction * jt.signum())
            };

            body_a.vel -= friction_impulse * body_a.inv_mass;
            body_a.angular_vel -= ra.cross(friction_impulse) * body_a.inv_inertia;

            body_b.vel += friction_impulse * body_b.inv_mass;
            body_b.angular_vel += rb.cross(friction_impulse) * body_b.inv_inertia;
        }
    }

    let v_bias_a = body_a.bias_vel + ra.perp() * body_a.bias_angular_vel;
    let v_bias_b = body_b.bias_vel + rb.perp() * body_b.bias_angular_vel;
    let rv_bias = v_bias_b - v_bias_a;
    let bias_along_normal = rv_bias.dot(normal);

    let slop = 1.5;
    let percent = 0.4;
    let target_bias = (depth - slop).max(0.0) * percent / dt;

    let ra_cross_n = ra.cross(normal);
    let rb_cross_n = rb.cross(normal);

    let inv_mass_sum = body_a.inv_mass
        + body_b.inv_mass
        + ra_cross_n * ra_cross_n * body_a.inv_inertia
        + rb_cross_n * rb_cross_n * body_b.inv_inertia;

    if inv_mass_sum > 0.0 {
        let j_bias = (target_bias - bias_along_normal) / inv_mass_sum;
        let j_bias = j_bias.max(0.0);
        let bias_impulse = normal * j_bias;

        body_a.bias_vel -= bias_impulse * body_a.inv_mass;
        body_a.bias_angular_vel -= ra.cross(bias_impulse) * body_a.inv_inertia;

        body_b.bias_vel += bias_impulse * body_b.inv_mass;
        body_b.bias_angular_vel += rb.cross(bias_impulse) * body_b.inv_inertia;
    }
}
