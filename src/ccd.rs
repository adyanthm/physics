use crate::Vec2;
use crate::aabb::polygon_aabb;
use crate::sat::{polygon_axes, project_polygon};
struct AABB {
    min: Vec2,
    max: Vec2,
}

impl AABB {
    pub fn visual_swept_bounds(&self, vel: Vec2, dt: f32) -> AABB {
        let disp = vel * dt;

        AABB {
            min: Vec2::new(
                self.min.x.min(self.min.x + disp.x),
                self.min.y.min(self.min.y + disp.y),
            ),
            max: Vec2::new(
                self.max.x.max(self.max.x + disp.x),
                self.max.y.max(self.max.y + disp.y),
            ),
        }
    }
}

pub struct CCDResult {
    pub toi: f32,
    pub normal: Vec2,
}

pub fn swept_sat(
    poly1: &[Vec2],
    vel_a: Vec2,
    poly2: &[Vec2],
    vel_b: Vec2,
    axes: &[Vec2],
    dt: f32,
) -> Option<CCDResult> {
    let mut t_first = 0.0;
    let mut t_last: f32 = 1.0;
    let mut hit_normal = Vec2::ZERO;

    let rel_disp = (vel_a - vel_b) * dt;

    for &axis in axes {
        let (min_a, max_a) = project_polygon(axis, poly1);
        let (min_b, max_b) = project_polygon(axis, poly2);

        let speed = rel_disp.dot(axis);

        if speed.abs() < f32::EPSILON {
            if max_b < min_a || max_a < min_b {
                return None;
            }
            continue;
        }
        let mut t0 = (min_b - max_a) / speed;
        let mut t1 = (max_b - min_a) / speed;

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        if t0 > t_first {
            t_first = t0;
            hit_normal = axis;
        }

        t_last = t_last.min(t1);

        if t_first > t_last {
            return None;
        }
    }
    if t_first >= 0.0 && t_first <= 1.0 {
        Some(CCDResult {
            toi: t_first,
            normal: hit_normal,
        })
    } else {
        None
    }
}

pub fn fast_poly_collide(
    poly1: &[Vec2],
    poly2: &[Vec2],
    vel_a: Vec2,
    vel_b: Vec2,
    dt: f32,
) -> Option<CCDResult> {
    let (min_x_a, min_y_a, max_x_a, max_y_a) = polygon_aabb(poly1);
    let aabb_a = AABB {
        min: Vec2::new(min_x_a, min_y_a),
        max: Vec2::new(max_x_a, max_y_a),
    };

    let (min_x_b, min_y_b, max_x_b, max_y_b) = polygon_aabb(poly2);
    let aabb_b = AABB {
        min: Vec2::new(min_x_b, min_y_b),
        max: Vec2::new(max_x_b, max_y_b),
    };

    let swept_a = aabb_a.visual_swept_bounds(vel_a, dt);
    let swept_b = aabb_b.visual_swept_bounds(vel_b, dt);

    if swept_a.max.x < swept_b.min.x
        || swept_b.max.x < swept_a.min.x
        || swept_a.max.y < swept_b.min.y
        || swept_b.max.y < swept_a.min.y
    {
        return None;
    }

    let mut axes = polygon_axes(poly1);
    axes.extend(polygon_axes(poly2));
    if let Some(mut result) = swept_sat(poly1, vel_a, poly2, vel_b, &axes, dt) {
        result.normal = -result.normal;
        Some(result)
    } else {
        None
    }
}
