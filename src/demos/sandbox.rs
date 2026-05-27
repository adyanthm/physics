use macroquad::prelude::*;

use crate::Vec2;
use crate::body::{RigidBody, rect_vertices};
use crate::world::PhysicsWorld;
use macroquad::rand::gen_range;

fn draw_polygon_filled(vertices: &[Vec2], fill_color: Color, outline_color: Color) {
    if vertices.len() < 3 {
        return;
    }
    for i in 1..(vertices.len() - 1) {
        draw_triangle(
            macroquad::math::Vec2::new(vertices[0].x, vertices[0].y),
            macroquad::math::Vec2::new(vertices[i].x, vertices[i].y),
            macroquad::math::Vec2::new(vertices[i + 1].x, vertices[i + 1].y),
            fill_color,
        );
    }
    for i in 0..vertices.len() {
        let v1 = vertices[i];
        let v2 = vertices[(i + 1) % vertices.len()];
        draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, outline_color);
    }
}

fn regular_polygon(radius: f32, sides: usize) -> Vec<Vec2> {
    let mut vertices = Vec::new();
    let angle_step = std::f32::consts::PI * 2.0 / sides as f32;
    for i in 0..sides {
        let angle = i as f32 * angle_step;
        vertices.push(Vec2::new(radius * angle.cos(), radius * angle.sin()));
    }
    vertices
}

fn random_pastel_color() -> Color {
    Color::new(
        gen_range(0.6_f32, 1.0_f32),
        gen_range(0.6_f32, 1.0_f32),
        gen_range(0.6_f32, 1.0_f32),
        1.0,
    )
}

struct RenderBody {
    handle: usize,
    color: Color,
}

pub async fn run() {
    let mut world = PhysicsWorld::new();
    let mut render_bodies = Vec::new();

    let mut left_wall = RigidBody::new_static(Vec2::new(100.0, 300.0), rect_vertices(50.0, 400.0));
    left_wall.restitution = 0.2;
    world.add_body(left_wall);

    let mut right_wall = RigidBody::new_static(Vec2::new(700.0, 300.0), rect_vertices(50.0, 400.0));
    right_wall.restitution = 0.2;
    world.add_body(right_wall);

    let mut floor = RigidBody::new_static(Vec2::new(400.0, 525.0), rect_vertices(650.0, 50.0));
    floor.restitution = 0.2;
    world.add_body(floor);

    let mut spawn_timer = 0.0;

    loop {
        clear_background(BLACK);
        let dt = 1.0 / 60.0;

        spawn_timer += get_frame_time();
        if spawn_timer > 0.05 && render_bodies.len() < 30 {
            spawn_timer = 0.0;
            let sides = gen_range(3, 7);
            let radius = gen_range(10.0, 25.0);

            let vertices = if sides == 4 {
                rect_vertices(radius * 1.5, radius * 1.5)
            } else {
                regular_polygon(radius, sides)
            };

            let mut body =
                RigidBody::new_dynamic(Vec2::new(gen_range(200.0, 600.0), 50.0), vertices);
            body.restitution = 0.4;
            body.angle = gen_range(0.0, std::f32::consts::TAU);
            body.angular_vel = gen_range(-5.0, 5.0);

            let handle = world.add_body(body);
            render_bodies.push(RenderBody {
                handle,
                color: random_pastel_color(),
            });
        }

        world.step(dt);

        for (i, body) in world.bodies.iter().enumerate() {
            let (fill, outline) = if let Some(rb) = render_bodies.iter().find(|rb| rb.handle == i) {
                (rb.color, WHITE)
            } else {
                (DARKGRAY, LIGHTGRAY)
            };
            draw_polygon_filled(&body.world_vertices(), fill, outline);
        }

        draw_text(
            &format!("Bodies: {}", render_bodies.len()),
            20.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text("Press ESC to exit", 20.0, 60.0, 20.0, GRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
