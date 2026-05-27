use crate::{Vec2, aabb_overlap};

pub struct Collision {
    pub normal: Vec2,
    pub depth: f32,
    pub contacts: Vec<Vec2>,
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

fn best_edge(polygon: &[Vec2], normal: Vec2) -> (Vec2, Vec2) {
    let mut max_dot = -f32::INFINITY;
    let mut best_idx = 0;
    for i in 0..polygon.len() {
        let d = polygon[i].dot(normal);
        if d > max_dot {
            max_dot = d;
            best_idx = i;
        }
    }

    let prev_idx = if best_idx == 0 {
        polygon.len() - 1
    } else {
        best_idx - 1
    };
    let next_idx = (best_idx + 1) % polygon.len();

    let v_best = polygon[best_idx];
    let v_prev = polygon[prev_idx];
    let v_next = polygon[next_idx];

    let edge_prev = (v_best - v_prev).normalize();
    let edge_next = (v_next - v_best).normalize();

    if edge_prev.dot(normal) <= edge_next.dot(normal) {
        (v_prev, v_best)
    } else {
        (v_best, v_next)
    }
}

fn clip(v1: Vec2, v2: Vec2, n: Vec2, o: f32) -> Vec<Vec2> {
    let mut cp = Vec::new();
    let d1 = n.dot(v1) - o;
    let d2 = n.dot(v2) - o;

    if d1 >= 0.0 {
        cp.push(v1);
    }
    if d2 >= 0.0 {
        cp.push(v2);
    }
    if d1 * d2 < 0.0 {
        let t = d1 / (d1 - d2);
        cp.push(v1 + (v2 - v1) * t);
    }
    cp
}

fn compute_contact_points(poly1: &[Vec2], poly2: &[Vec2], normal: Vec2) -> Vec<Vec2> {
    let (e1_v1, e1_v2) = best_edge(poly1, -normal);
    let (e2_v1, e2_v2) = best_edge(poly2, normal);

    let ref_edge;
    let inc_edge;
    let mut flip = false;

    if (e1_v2 - e1_v1).normalize().dot(normal).abs()
        <= (e2_v2 - e2_v1).normalize().dot(normal).abs()
    {
        ref_edge = (e1_v1, e1_v2);
        inc_edge = (e2_v1, e2_v2);
    } else {
        ref_edge = (e2_v1, e2_v2);
        inc_edge = (e1_v1, e1_v2);
        flip = true;
    }

    let ref_v = (ref_edge.1 - ref_edge.0).normalize();
    let o1 = ref_v.dot(ref_edge.0);
    let cp = clip(inc_edge.0, inc_edge.1, ref_v, o1);
    if cp.len() < 2 {
        return vec![(ref_edge.0 + ref_edge.1 + inc_edge.0 + inc_edge.1) * 0.25];
    }

    let o2 = ref_v.dot(ref_edge.1);
    let cp = clip(cp[0], cp[1], -ref_v, -o2);
    if cp.len() < 2 {
        return vec![(ref_edge.0 + ref_edge.1 + inc_edge.0 + inc_edge.1) * 0.25];
    }

    let ref_normal = if flip { normal } else { -normal };
    let max_depth = ref_normal.dot(ref_edge.0);

    let mut valid_cp = Vec::new();
    for v in cp {
        if ref_normal.dot(v) - max_depth <= 0.1 {
            valid_cp.push(v);
        }
    }

    if valid_cp.is_empty() {
        return vec![(ref_edge.0 + ref_edge.1 + inc_edge.0 + inc_edge.1) * 0.25];
    }

    valid_cp
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

    let contacts = compute_contact_points(poly1, poly2, smallest_axis);

    Some(Collision {
        normal: smallest_axis,
        depth: smallest_overlap,
        contacts,
    })
}

pub fn resolve_pos(
    pos1: &mut Vec2,
    pos2: &mut Vec2,
    normal: Vec2,
    depth: f32,
    inv_mass1: f32,
    inv_mass2: f32,
) {
    if depth <= 0.0 {
        return;
    }
    let total_inv_mass = inv_mass1 + inv_mass2;
    if total_inv_mass == 0.0 {
        return;
    }
    let correction = normal * (depth / total_inv_mass);
    *pos1 += correction * inv_mass1;
    *pos2 -= correction * inv_mass2;
}

pub fn resolve_pos_static(pos1: &mut Vec2, normal: Vec2, depth: f32) {
    if depth <= 0.0 {
        return;
    }
    *pos1 += normal * depth;
}
