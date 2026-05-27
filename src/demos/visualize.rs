use macroquad::prelude::*;

use crate::Vec2;
use crate::body::{RigidBody, rect_vertices};
use crate::world::PhysicsWorld;

fn draw_polygon_outline(vertices: &[Vec2], color: Color) {
    if vertices.len() < 3 {
        return;
    }
    for i in 0..vertices.len() {
        let v1 = vertices[i];
        let v2 = vertices[(i + 1) % vertices.len()];
        draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, color);
    }
}

pub async fn run() {
    let mut world = PhysicsWorld::new();

    let mut floor = RigidBody::new_static(Vec2::new(400.0, 500.0), rect_vertices(600.0, 50.0));
    floor.restitution = 0.5;
    world.add_body(floor);

    let mut box_body = RigidBody::new_dynamic(Vec2::new(400.0, 100.0), rect_vertices(50.0, 50.0));
    box_body.restitution = 0.7;
    box_body.angle = std::f32::consts::PI / 6.0;
    box_body.angular_vel = 2.0;
    let box_id = world.add_body(box_body);

    loop {
        clear_background(BLACK);
        world.step(1.0 / 60.0);

        for (i, body) in world.bodies.iter().enumerate() {
            let color = if i == box_id { GREEN } else { GRAY };
            draw_polygon_outline(&body.world_vertices(), color);

            if i == box_id {
                draw_line(
                    body.pos.x,
                    body.pos.y,
                    body.pos.x + body.vel.x * 0.1,
                    body.pos.y + body.vel.y * 0.1,
                    2.0,
                    BLUE,
                );
                let r = 30.0;
                let av_x = body.pos.x + r * body.angle.cos();
                let av_y = body.pos.y + r * body.angle.sin();
                draw_line(body.pos.x, body.pos.y, av_x, av_y, 2.0, PURPLE);
                draw_circle(av_x, av_y, 4.0, PURPLE);
            }
        }

        for c in &world.contacts {
            if c.body_a == box_id || c.body_b == box_id {
                for &contact in &c.contacts {
                    draw_circle(contact.x, contact.y, 4.0, RED);

                    draw_line(
                        contact.x,
                        contact.y,
                        contact.x + c.normal.x * 30.0,
                        contact.y + c.normal.y * 30.0,
                        2.0,
                        YELLOW,
                    );
                }
            }
        }

        draw_text("Single Block Gravity Demo", 20.0, 30.0, 30.0, WHITE);
        draw_text("Blue Line: Velocity", 20.0, 60.0, 20.0, BLUE);
        draw_text("Purple Line: Rotation Angle", 20.0, 85.0, 20.0, PURPLE);
        draw_text("Red Dot: Contact Points", 20.0, 110.0, 20.0, RED);
        draw_text("Yellow Line: Collision Normal", 20.0, 135.0, 20.0, YELLOW);

        draw_text("Press 'R' to reset the box", 20.0, 175.0, 20.0, WHITE);
        draw_text("Press ESC to exit", 20.0, 200.0, 20.0, GRAY);

        if is_key_pressed(KeyCode::R) {
            world.bodies[box_id].pos = Vec2::new(400.0, 100.0);
            world.bodies[box_id].vel = Vec2::ZERO;
            world.bodies[box_id].angle = std::f32::consts::PI / 6.0;
            world.bodies[box_id].angular_vel = 2.0;
            world.bodies[box_id].bias_vel = Vec2::ZERO;
            world.bodies[box_id].bias_angular_vel = 0.0;
            world.bodies[box_id].awake = true;
            world.bodies[box_id].sleep_timer = 0.0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
