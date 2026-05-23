use crate::Vec2;
use crate::aabb::polygon_aabb;
use crate::sat::{polygon_axes, project_polygon};
use crate::vectors::{dot, scale, sub};
struct AABB {
    min: Vec2,
    max: Vec2,
}

impl AABB {
    pub fn visual_swept_bounds(&self, vel: Vec2, dt: f32) -> AABB {
        let disp_x = vel.0 * dt;
        let disp_y = vel.1 * dt;

        AABB {
            min: (
                self.min.0.min(self.min.0 + disp_x), // self.min means struct AABB's min. not min().
                self.min.1.min(self.min.1 + disp_y),
            ),
            max: (
                self.max.0.max(self.max.0 + disp_x),
                self.max.1.max(self.max.1 + disp_y),
            ),
        }
    }
}

pub struct CCDResult {
    pub toi: f32, // time of impact (0-1)
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
    let mut hit_normal = (0.0, 0.0);

    let rel_disp = scale(sub(vel_a, vel_b), dt);

    for &axis in axes {
        let (min_a, max_a) = project_polygon(axis, poly1);
        let (min_b, max_b) = project_polygon(axis, poly2);

        let speed = dot(rel_disp, axis);

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
        min: (min_x_a, min_y_a),
        max: (max_x_a, max_y_a),
    };

    let (min_x_b, min_y_b, max_x_b, max_y_b) = polygon_aabb(poly2);
    let aabb_b = AABB {
        min: (min_x_b, min_y_b),
        max: (max_x_b, max_y_b),
    };

    let swept_a = aabb_a.visual_swept_bounds(vel_a, dt);
    let swept_b = aabb_b.visual_swept_bounds(vel_b, dt);

    if swept_a.max.0 < swept_b.min.0
        || swept_b.max.0 < swept_a.min.0
        || swept_a.max.1 < swept_b.min.1
        || swept_b.max.1 < swept_a.min.1
    {
        return None;
    }

    let mut axes = polygon_axes(poly1);
    axes.extend(polygon_axes(poly2));
    if let Some(mut result) = swept_sat(poly1, vel_a, poly2, vel_b, &axes, dt) {
        result.normal = (-result.normal.0, -result.normal.1);
        Some(result)
    } else {
        None
    }
}
