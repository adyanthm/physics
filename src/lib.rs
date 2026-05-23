pub mod aabb;
pub mod ccd;
pub mod concave;
pub mod demos;
pub mod point;
pub mod sat;
pub mod vectors;
pub mod velocity;
pub use aabb::aabb_overlap;
pub use concave::polygon_concave;
pub use point::point_polygon;
pub use sat::{advanced_collision, polygon_collision, resolve_pos, resolve_pos_static};
pub use vectors::{Vec2, dot, normalize};
pub use velocity::{Body, GRAVITY, resolve_velocity, update};
pub use ccd::{fast_poly_collide};

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> Vec<Vec2> {
        vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
    }

    fn offset_square() -> Vec<Vec2> {
        vec![(3.0, 1.0), (7.0, 1.0), (7.0, 5.0), (3.0, 5.0)]
    }

    fn far_square() -> Vec<Vec2> {
        vec![(10.0, 10.0), (14.0, 10.0), (14.0, 14.0), (10.0, 14.0)]
    }

    fn l_shape() -> Vec<Vec2> {
        vec![
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ]
    }

    // --- vectors: dot ---

    #[test]
    fn dot_perpendicular_is_zero() {
        assert_eq!(dot((1.0, 0.0), (0.0, 1.0)), 0.0);
    }

    #[test]
    fn dot_parallel() {
        assert_eq!(dot((2.0, 3.0), (2.0, 3.0)), 13.0);
    }

    // --- vectors: normalize ---

    #[test]
    fn normalize_unit_vector() {
        let (x, y) = normalize(3.0, 4.0);
        assert!((x - 0.6).abs() < 1e-6);
        assert!((y - 0.8).abs() < 1e-6);
    }

    #[test]
    fn normalize_zero_vector() {
        assert_eq!(normalize(0.0, 0.0), (0.0, 0.0));
    }

    // --- point_polygon ---

    #[test]
    fn point_inside() {
        assert!(point_polygon(2.0, 2.0, &square()));
    }

    #[test]
    fn point_outside() {
        assert!(!point_polygon(10.0, 10.0, &square()));
    }

    #[test]
    // on edge != collision for now. only strictly inside counts.
    fn point_on_edge() {
        assert!(!point_polygon(4.0, 2.0, &square()));
    }

    #[test]
    fn point_empty_polygon() {
        assert!(!point_polygon(0.0, 0.0, &[]));
    }

    // --- aabb_overlap ---

    #[test]
    fn aabb_overlapping() {
        assert!(aabb_overlap(&square(), &offset_square()));
    }

    #[test]
    fn aabb_disjoint() {
        assert!(!aabb_overlap(&square(), &far_square()));
    }

    #[test]
    fn aabb_empty_polygon() {
        assert!(!aabb_overlap(&square(), &[]));
    }

    // --- polygon_collision (convex, sat) ---

    #[test]
    fn sat_colliding() {
        assert!(polygon_collision(&square(), &offset_square()));
    }

    #[test]
    fn sat_not_colliding() {
        assert!(!polygon_collision(&square(), &far_square()));
    }

    #[test]
    fn sat_identical_shapes() {
        assert!(polygon_collision(&square(), &square()));
    }

    // --- advanced_collision (MTV) ---

    #[test]
    fn mtv_returns_collision() {
        let (hit, _normal, depth) = advanced_collision(&square(), &offset_square());
        assert!(hit);
        assert!(depth > 0.0);
    }

    #[test]
    fn mtv_resolves_overlap() {
        let (hit, normal, depth) = advanced_collision(&square(), &offset_square());
        assert!(hit);
        let resolved: Vec<Vec2> = offset_square()
            .iter()
            .map(|&(x, y)| (x + normal.0 * depth, y + normal.1 * depth))
            .collect();
        assert!(!polygon_collision(&square(), &resolved));
    }

    #[test]
    fn mtv_no_collision() {
        let (hit, _, depth) = advanced_collision(&square(), &far_square());
        assert!(!hit);
        assert_eq!(depth, 0.0);
    }

    // --- polygon_concave ---

    #[test]
    fn concave_colliding() {
        let tri = vec![(0.5, 0.5), (2.0, 2.0), (0.5, 2.0)];
        assert!(polygon_concave(&l_shape(), &tri));
    }

    #[test]
    fn concave_not_colliding() {
        assert!(!polygon_concave(&l_shape(), &far_square()));
    }
}
