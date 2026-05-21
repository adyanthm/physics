use crate::Vec2;
pub fn point_polygon(x: f32, y: f32, polygon: &[Vec2]) -> bool {
    if polygon.is_empty() {
        return false;
    }
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        let intersects = (yi > y) != (yj > y);
        if intersects {
            let intersection_x = (xj - xi) * (y - yi) / (yj - yi) + xi;
            if intersection_x > x {
                inside = !inside
            }
        }
        j = i
    }
    inside
}
