pub mod aabb;
pub mod concave;
pub mod point;
pub mod sat;
pub mod vectors;
pub use aabb::aabb_overlap;
pub use point::point_polygon;
pub use sat::polygon_collision;
pub use vectors::{Vec2, dot, normalize};

#[cfg(test)]
mod tests {
    use super::*;
    fn square() -> Vec<(f32, f32)> {
        vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
    }
    #[test]
    fn point_in_square() {
        assert!(point_polygon(2.0, 2.0, &square()))
    }
    #[test]
    fn point_outside_square() {
        assert!(!point_polygon(10.0, 10.0, &square()))
    }
    #[test]
    // I have considered on edge != collision. Only inside = collision. IL implement on edge check later
    fn point_on_edge_square() {
        assert!(!point_polygon(4.0, 2.0, &square()))
    }
}
