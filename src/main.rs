use std::time::Instant;

use collision::{Vec2, polygon_collision};

fn generate_polygon(sides: usize, radius: f32, offset_x: f32, offset_y: f32) -> Vec<Vec2> {
    let mut points = Vec::new();

    for i in 0..sides {
        let angle = (i as f32 / sides as f32) * std::f32::consts::TAU;

        let x = offset_x + radius * angle.cos();
        let y = offset_y + radius * angle.sin();

        points.push((x, y));
    }

    points
}

fn main() {
    const ITERATIONS: usize = 1_000_000;

    let polygon_sizes = [3, 4, 5, 8, 16, 32];

    for &size in &polygon_sizes {
        let poly1 = generate_polygon(size, 50.0, 0.0, 0.0);
        let poly2 = generate_polygon(size, 50.0, 25.0, 25.0);

        let start = Instant::now();

        let mut collisions = 0;

        for _ in 0..ITERATIONS {
            if polygon_collision(&poly1, &poly2) {
                collisions += 1;
            }
        }

        let elapsed = start.elapsed();

        println!("Polygon size: {}", size);
        println!("Iterations: {}", ITERATIONS);
        println!("Collisions: {}", collisions);
        println!("Elapsed: {:?}", elapsed);

        println!(
            "Average per collision check: {:.4} ns",
            elapsed.as_nanos() as f64 / ITERATIONS as f64
        );

        println!("--------------------------------------");
    }
}
