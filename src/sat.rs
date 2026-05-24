use crate::{Vec2, aabb_overlap};

pub struct Collision {
    pub normal: Vec2,
    pub depth: f32,
}
pub fn project_polygon(axis: Vec2, polygon: &[Vec2]) -> (f32, f32) {
    if polygon.is_empty() {
        return (0.0, 0.0);
    }
    let mut min = polygon[0].dot(axis);
    let mut max = min;
    for &point in polygon {
        let projection = point.dot(axis);
        min = min.min(projection);
        max = max.max(projection);
    }
    (min, max)
}

fn overlap(min_a: f32, min_b: f32, max_a: f32, max_b: f32) -> bool {
    max_a >= min_b && max_b >= min_a
}

pub fn polygon_axes(polygon: &[Vec2]) -> Vec<Vec2> {
    let mut axes = Vec::new();
    for i in 0..polygon.len() {
        let edge = polygon[i] - polygon[(i + 1) % polygon.len()];
        let axis = edge.perp().normalize();
        axes.push(axis);
    }
    axes
}

pub fn polygon_collision(poly1: &[Vec2], poly2: &[Vec2]) -> bool {
    if !aabb_overlap(poly1, poly2) {
        return false;
    }
    let mut axes = polygon_axes(poly1);
    axes.extend(polygon_axes(poly2));
    for axis in axes {
        let (min_a, max_a) = project_polygon(axis, poly1);
        let (min_b, max_b) = project_polygon(axis, poly2);
        if !overlap(min_a, min_b, max_a, max_b) {
            return false;
        }
    }

    true
}

fn polygon_center(polygon: &[Vec2]) -> Vec2 {
    if polygon.is_empty() {
        return Vec2::ZERO;
    }
    let mut total = Vec2::ZERO;
    for &v in polygon {
        total += v;
    }

    total / polygon.len() as f32
}

pub fn advanced_collision(poly1: &[Vec2], poly2: &[Vec2]) -> Option<Collision> {
    let mut smallest_overlap = f32::INFINITY;
    let mut smallest_axis = Vec2::ZERO;
    if !aabb_overlap(poly1, poly2) {
        return None;
    }
    let axes = polygon_axes(poly1).into_iter().chain(polygon_axes(poly2));
    for axis in axes {
        let (min_a, max_a) = project_polygon(axis, poly1);
        let (min_b, max_b) = project_polygon(axis, poly2);
        let overlap_amount = max_a.min(max_b) - min_a.max(min_b);
        if overlap_amount <= 0.001 {
            return None;
        }
        if overlap_amount < smallest_overlap {
            smallest_overlap = overlap_amount;
            smallest_axis = axis;
        }
    }
    let direction = polygon_center(poly2) - polygon_center(poly1);
    if direction.dot(smallest_axis) > 0.0 {
        smallest_axis = -smallest_axis;
    }
    Some(Collision {
        normal: smallest_axis,
        depth: smallest_overlap,
    })
}

pub fn resolve_pos(pos1: &mut Vec2, pos2: &mut Vec2, normal: Vec2, depth: f32) {
    if depth <= 0.0 {
        return;
    }
    let correction = normal * depth;
    *pos1 += correction * 0.5;
    *pos2 -= correction * 0.5;
}

pub fn resolve_pos_static(pos1: &mut Vec2, normal: Vec2, depth: f32) {
    if depth <= 0.0 {
        return;
    }
    *pos1 += normal * depth;
}
