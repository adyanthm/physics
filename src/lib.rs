pub mod aabb;
pub mod body;
pub mod ccd;
pub mod concave;
pub mod demos;
pub mod point;
pub mod sat;
pub mod vectors;
pub mod velocity;
pub mod world;
pub use aabb::aabb_overlap;
pub use ccd::fast_poly_collide;
pub use concave::polygon_concave;
pub use point::point_polygon;
pub use sat::{Collision, advanced_collision, polygon_collision, resolve_pos, resolve_pos_static};
pub use vectors::Vec2;
pub use velocity::resolve_collision;

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> Vec<Vec2> {
        vec![
            Vec2::new(-2.0, -2.0),
            Vec2::new(2.0, -2.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(-2.0, 2.0),
        ]
    }

    fn offset_square() -> Vec<Vec2> {
        vec![
            Vec2::new(1.0, -1.0),
            Vec2::new(5.0, -1.0),
            Vec2::new(5.0, 3.0),
            Vec2::new(1.0, 3.0),
        ]
    }

    fn far_square() -> Vec<Vec2> {
        vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(14.0, 10.0),
            Vec2::new(14.0, 14.0),
            Vec2::new(10.0, 14.0),
        ]
    }

    fn l_shape() -> Vec<Vec2> {
        vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 0.0),
            Vec2::new(3.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 3.0),
            Vec2::new(0.0, 3.0),
        ]
    }

    #[test]
    fn dot_perpendicular_is_zero() {
        assert_eq!(Vec2::new(1.0, 0.0).dot(Vec2::new(0.0, 1.0)), 0.0);
    }

    #[test]
    fn dot_parallel() {
        assert_eq!(Vec2::new(2.0, 3.0).dot(Vec2::new(2.0, 3.0)), 13.0);
    }

    #[test]
    fn normalize_unit_vector() {
        let n = Vec2::new(3.0, 4.0).normalize();
        assert!((n.x - 0.6).abs() < 1e-6);
        assert!((n.y - 0.8).abs() < 1e-6);
    }

    #[test]
    fn normalize_zero_vector() {
        assert_eq!(Vec2::ZERO.normalize(), Vec2::ZERO);
    }

    #[test]
    fn point_inside() {
        assert!(point_polygon(Vec2::new(0.0, 0.0), &square()));
    }

    #[test]
    fn point_outside() {
        assert!(!point_polygon(Vec2::new(10.0, 10.0), &square()));
    }

    #[test]
    fn point_on_edge() {
        assert!(!point_polygon(Vec2::new(2.0, 0.0), &square()));
    }

    #[test]
    fn point_empty_polygon() {
        assert!(!point_polygon(Vec2::ZERO, &[]));
    }

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

    #[test]
    fn mtv_returns_collision() {
        let result = advanced_collision(&square(), &offset_square());
        assert!(result.is_some());
        assert!(result.unwrap().depth > 0.0);
    }

    #[test]
    fn mtv_resolves_overlap() {
        let Collision { normal, depth, .. } =
            advanced_collision(&square(), &offset_square()).unwrap();
        let resolved: Vec<Vec2> = square().iter().map(|&v| v + normal * depth).collect();
        assert!(!polygon_collision(&resolved, &offset_square()));
    }

    #[test]
    fn mtv_no_collision() {
        assert!(advanced_collision(&square(), &far_square()).is_none());
    }

    #[test]
    fn concave_colliding() {
        let tri = vec![
            Vec2::new(0.5, 0.5),
            Vec2::new(2.0, 2.0),
            Vec2::new(0.5, 2.0),
        ];
        assert!(polygon_concave(&l_shape(), &tri));
    }

    #[test]
    fn concave_not_colliding() {
        assert!(!polygon_concave(&l_shape(), &far_square()));
    }
}
