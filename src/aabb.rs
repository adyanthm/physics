use crate::Vec2;
pub fn polygon_aabb(polygon: &[Vec2]) -> (f32, f32, f32, f32) {
    let (mut min_x, mut min_y) = polygon[0];
    let (mut max_x, mut max_y) = polygon[0];
    for &(x, y) in polygon {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    (min_x, min_y, max_x, max_y)
}

pub fn aabb_overlap(poly_a: &[Vec2], poly_b: &[Vec2]) -> bool {
    if poly_a.is_empty() || poly_b.is_empty() {
        return false;
    }
    let a = polygon_aabb(poly_a);
    let b = polygon_aabb(poly_b);
    let (a_min_x, a_min_y, a_max_x, a_max_y) = a;
    let (b_min_x, b_min_y, b_max_x, b_max_y) = b;
    a_min_x < b_max_x && a_max_x > b_min_x && a_min_y < b_max_y && a_max_y > b_min_y
}
