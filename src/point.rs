use crate::Vec2;
pub fn point_polygon(point: Vec2, polygon: &[Vec2]) -> bool {
    if polygon.is_empty() {
        return false;
    }
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let pi = polygon[i];
        let pj = polygon[j];
        let intersects = (pi.y > point.y) != (pj.y > point.y);
        if intersects {
            let intersection_x = (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x;
            if intersection_x > point.x {
                inside = !inside
            }
        }
        j = i
    }
    inside
}
