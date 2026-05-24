use crate::{Vec2, aabb_overlap, point_polygon};

fn point_side(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (b - a).cross(c - a)
}

fn segment_intersect(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    let ab_c = point_side(a, b, c);
    let ab_d = point_side(a, b, d);
    let cd_a = point_side(c, d, a);
    let cd_b = point_side(c, d, b);
    (ab_c * ab_d <= 0.0) && (cd_a * cd_b <= 0.0)
}

pub fn polygon_concave(poly1: &[Vec2], poly2: &[Vec2]) -> bool {
    if !aabb_overlap(poly1, poly2) {
        return false;
    }
    for i in 0..poly1.len() {
        let a = poly1[i];
        let b = poly1[(i + 1) % poly1.len()];
        for j in 0..poly2.len() {
            let c = poly2[j];
            let d = poly2[(j + 1) % poly2.len()];
            if segment_intersect(a, b, c, d) {
                return true;
            }
        }
    }
    for &v in poly1 {
        if point_polygon(v, poly2) {
            return true;
        }
    }
    for &v in poly2 {
        if point_polygon(v, poly1) {
            return true;
        }
    }

    false
}
