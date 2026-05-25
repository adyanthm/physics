use macroquad::prelude::*;

use crate::Vec2;
use crate::body::{BodyType, RigidBody, rect_vertices};
use crate::world::PhysicsWorld;

pub async fn run() {
    let mut world = PhysicsWorld::new();

    let box_handle = world.add_body(RigidBody {
        pos: Vec2::new(350.0, 50.0),
        vel: Vec2::new(0.0, 0.0),
        body_type: BodyType::Dynamic,
        use_gravity: true,
        restitution: 0.7,
        vertices: rect_vertices(50.0, 50.0),
    });
    world.add_body(RigidBody::new_static(
        Vec2::new(100.0, 500.0),
        rect_vertices(600.0, 20.0),
    ));

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        world.step(dt);

        let body = &world.bodies[box_handle];
        draw_rectangle(body.pos.x, body.pos.y, 50.0, 50.0, BLUE);
        draw_rectangle(100.0, 500.0, 600.0, 20.0, GRAY);

        draw_line(
            body.pos.x + 25.0,
            body.pos.y + 25.0,
            body.pos.x + 25.0 + body.vel.x * 0.1,
            body.pos.y + 25.0 + body.vel.y * 0.1,
            3.0,
            RED,
        );

        if let Some(c) = world
            .contacts
            .iter()
            .find(|c| c.body_a == box_handle || c.body_b == box_handle)
        {
            let normal = if c.body_a == box_handle {
                c.normal
            } else {
                -c.normal
            };
            draw_line(
                body.pos.x + 25.0,
                body.pos.y + 25.0,
                body.pos.x + 25.0 + normal.x * 100.0,
                body.pos.y + 25.0 + normal.y * 100.0,
                4.0,
                YELLOW,
            );
        }

        let vel_text = format!("vel : ({:.1}, {:.1})", body.vel.x, body.vel.y);
        draw_text(&vel_text, 20.0, 30.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
