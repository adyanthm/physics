use macroquad::prelude::*;

use crate::Vec2;
use crate::body::{RigidBody, rect_vertices};
use crate::world::PhysicsWorld;

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

pub async fn run() {
    let player_size = 30.0;
    let speed = 400.0;
    let jump_force = 500.0;

    let mut world = PhysicsWorld::new();

    let mut player_body = RigidBody::new_dynamic(
        Vec2::new(100.0, 100.0),
        rect_vertices(player_size, player_size),
    );
    player_body.friction = 0.0;
    let player = world.add_body(player_body);

    world.add_body(RigidBody::new_static(
        Vec2::new(425.0, 525.0),
        rect_vertices(750.0, 50.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(250.0, 410.0),
        rect_vertices(100.0, 20.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(475.0, 310.0),
        rect_vertices(150.0, 20.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(700.0, 210.0),
        rect_vertices(100.0, 20.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(75.0, 300.0),
        rect_vertices(50.0, 400.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(775.0, 300.0),
        rect_vertices(50.0, 400.0),
    ));

    loop {
        clear_background(BLACK);
        let dt = 1.0 / 60.0;

        let pb = &mut world.bodies[player];
        pb.angle = 0.0;
        pb.angular_vel = 0.0;

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            pb.vel.x = speed;
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            pb.vel.x = -speed;
        } else {
            pb.vel.x *= 0.9;
        }

        world.step(dt);

        if is_key_pressed(KeyCode::Space) && world.is_grounded(player) {
            world.bodies[player].vel.y = -jump_force;
        }

        for (i, body) in world.bodies.iter().enumerate() {
            let color = if i == player { BLUE } else { GRAY };
            draw_polygon_filled(&body.world_vertices(), color, color);
        }

        draw_text("Platformer Demo", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            "Left/Right or A/D to move, Space to jump, ESC to menu",
            20.0,
            60.0,
            20.0,
            GRAY,
        );

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
