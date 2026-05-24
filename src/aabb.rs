use crate::Vec2;
pub fn polygon_aabb(polygon: &[Vec2]) -> (f32, f32, f32, f32) {
    let mut min_x = polygon[0].x;
    let mut min_y = polygon[0].y;
    let mut max_x = polygon[0].x;
    let mut max_y = polygon[0].y;
    for &v in polygon {
        min_x = min_x.min(v.x);
        min_y = min_y.min(v.y);
        max_x = max_x.max(v.x);
        max_y = max_y.max(v.y);
    }
    (min_x, min_y, max_x, max_y)
}

pub fn aabb_overlap(poly_a: &[Vec2], poly_b: &[Vec2]) -> bool {
    if poly_a.is_empty() || poly_b.is_empty() {
        return false;
    }
    let (a_min_x, a_min_y, a_max_x, a_max_y) = polygon_aabb(poly_a);
    let (b_min_x, b_min_y, b_max_x, b_max_y) = polygon_aabb(poly_b);
    a_min_x < b_max_x && a_max_x > b_min_x && a_min_y < b_max_y && a_max_y > b_min_y
}
