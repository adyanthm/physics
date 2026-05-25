use macroquad::prelude::*;

use crate::Vec2;
use crate::body::{RigidBody, rect_vertices};
use crate::world::PhysicsWorld;

pub async fn run() {
    let player_size = 30.0;
    let speed = 400.0;
    let jump_force = 500.0;

    let mut world = PhysicsWorld::new();

    let player = world.add_body(RigidBody::new_dynamic(
        Vec2::new(100.0, 100.0),
        rect_vertices(player_size, player_size),
    ));

    world.add_body(RigidBody::new_static(
        Vec2::new(50.0, 500.0),
        rect_vertices(750.0, 50.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(200.0, 400.0),
        rect_vertices(100.0, 20.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(400.0, 300.0),
        rect_vertices(150.0, 20.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(650.0, 200.0),
        rect_vertices(100.0, 20.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(50.0, 100.0),
        rect_vertices(50.0, 400.0),
    ));
    world.add_body(RigidBody::new_static(
        Vec2::new(750.0, 100.0),
        rect_vertices(50.0, 400.0),
    ));

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        let pb = &mut world.bodies[player];

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
            let verts = body.world_vertices();
            let w = verts[1].x - verts[0].x;
            let h = verts[3].y - verts[0].y;
            let color = if i == player { BLUE } else { GRAY };
            draw_rectangle(body.pos.x, body.pos.y, w, h, color);
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
