use crate::{Vec2, aabb_overlap, dot, normalize};
pub fn project_polygon(axis: Vec2, polygon: &[Vec2]) -> (f32, f32) {
    if polygon.is_empty() {
        return (0.0, 0.0);
    }
    let mut min = dot(polygon[0], axis);
    let mut max = min;
    for &point in polygon {
        let projection = dot(point, axis);
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
        let (p1x, p1y) = polygon[i];
        let (p2x, p2y) = polygon[(i + 1) % polygon.len()];
        let edge = (p1x - p2x, p1y - p2y);
        let axis = normalize(-edge.1, edge.0);
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
    let mut total = (0.0, 0.0);
    for &(x, y) in polygon {
        total.0 += x;
        total.1 += y;
    }
    if polygon.is_empty() {
        return (0.0, 0.0);
    }
    let len = polygon.len() as f32;
    (total.0 / len, total.1 / len)
}

pub fn advanced_collision(poly1: &[Vec2], poly2: &[Vec2]) -> (bool, Vec2, f32) {
    let mut smallest_overlap = f32::INFINITY;
    let mut smallest_axis = (0.0, 0.0);
    if !aabb_overlap(poly1, poly2) {
        return (false, (0.0, 0.0), 0.0);
    }
    let axes = polygon_axes(poly1).into_iter().chain(polygon_axes(poly2));
    for axis in axes {
        let (min_a, max_a) = project_polygon(axis, poly1);
        let (min_b, max_b) = project_polygon(axis, poly2);
        let overlap_amount = max_a.min(max_b) - min_a.max(min_b);
        if overlap_amount <= 0.001 {
            return (false, (0.0, 0.0), 0.0);
        }
        if overlap_amount < smallest_overlap {
            smallest_overlap = overlap_amount;
            smallest_axis = axis;
        }
    }
    let center1 = polygon_center(poly1);
    let center2 = polygon_center(poly2);
    let direction = (center2.0 - center1.0, center2.1 - center1.1);
    if dot(direction, smallest_axis) < 0.0 {
        smallest_axis.0 = -smallest_axis.0;
        smallest_axis.1 = -smallest_axis.1;
    }
    (true, smallest_axis, smallest_overlap)
}

pub fn resolve_pos(pos1: &mut Vec2, pos2: &mut Vec2, normal: Vec2, depth: f32) {
    if depth <= 0.0 {
        return;
    }
    let correction = (normal.0 * depth, normal.1 * depth);
    pos1.0 -= correction.0 * 0.5;
    pos1.1 -= correction.1 * 0.5;
    pos2.0 += correction.0 * 0.5;
    pos2.1 += correction.1 * 0.5;
}

pub fn resolve_pos_static(pos1: &mut Vec2, normal: Vec2, depth: f32) {
    if depth <= 0.0 {
        return;
    }
    pos1.0 -= normal.0 * depth;
    pos1.1 -= normal.1 * depth;
}
