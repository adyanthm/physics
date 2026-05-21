use collision::{
    aabb_overlap,
    advanced_collision,
    point_polygon,
    polygon_collision,
    polygon_concave,
    Vec2,
};

fn main() {
    let square_a: Vec<Vec2> = vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)];
    let square_b: Vec<Vec2> = vec![(3.0, 1.0), (7.0, 1.0), (7.0, 5.0), (3.0, 5.0)];
    let square_c: Vec<Vec2> = vec![(10.0, 10.0), (14.0, 10.0), (14.0, 14.0), (10.0, 14.0)];

    let l_shape: Vec<Vec2> = vec![
        (0.0, 0.0), (3.0, 0.0), (3.0, 1.0),
        (1.0, 1.0), (1.0, 3.0), (0.0, 3.0),
    ];
    let triangle: Vec<Vec2> = vec![(0.5, 0.5), (2.0, 2.0), (0.5, 2.0)];

    // broadphase
    println!("--- AABB broadphase ---");
    println!("A & B overlap: {}", aabb_overlap(&square_a, &square_b));
    println!("A & C overlap: {}", aabb_overlap(&square_a, &square_c));

    // convex SAT
    println!("\n--- SAT (convex) ---");
    println!("A & B colliding: {}", polygon_collision(&square_a, &square_b));
    println!("A & C colliding: {}", polygon_collision(&square_a, &square_c));

    // MTV resolution
    println!("\n--- MTV resolution ---");
    let (hit, normal, depth) = advanced_collision(&square_a, &square_b);
    if hit {
        println!("normal: {:?}, depth: {}", normal, depth);
        let resolved: Vec<Vec2> = square_b.iter()
            .map(|&(x, y)| (x + normal.0 * depth, y + normal.1 * depth))
            .collect();
        println!("after push: colliding = {}", polygon_collision(&square_a, &resolved));
    }

    // concave
    println!("\n--- concave ---");
    println!("L-shape & triangle: {}", polygon_concave(&l_shape, &triangle));

    // point-in-polygon
    println!("\n--- point-in-polygon ---");
    println!("(2,2) in A: {}", point_polygon(2.0, 2.0, &square_a));
    println!("(5,5) in A: {}", point_polygon(5.0, 5.0, &square_a));
}
