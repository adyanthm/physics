use macroquad::prelude::*;

use crate::{Body, Vec2, advanced_collision, resolve_pos_static, resolve_velocity, update};

pub fn rect(pos: Vec2, size: f32) -> Vec<Vec2> {
    vec![
        Vec2::new(pos.x, pos.y),
        Vec2::new(pos.x + size, pos.y),
        Vec2::new(pos.x + size, pos.y + size),
        Vec2::new(pos.x, pos.y + size),
    ]
}

pub async fn run() {
    let mut body = Body {
        pos: Vec2::new(350.0, 50.0),
        vel: Vec2::ZERO,
        use_gravity: true,
    };

    let floor = [
        Vec2::new(100.0, 500.0),
        Vec2::new(700.0, 500.0),
        Vec2::new(700.0, 520.0),
        Vec2::new(100.0, 520.0),
    ];

    loop {
        clear_background(BLACK);
        let dt = get_frame_time() * 0.5;
        update(&mut body, dt);

        let player = rect(body.pos, 50.0);
        let collision = advanced_collision(&player, &floor);
        if let Some(c) = &collision {
            resolve_pos_static(&mut body.pos, c.normal, c.depth);
            resolve_velocity(&mut body.vel, c.normal, 0.0);

            if c.normal.y < -0.5 {
                body.pos.y -= 0.1;
                body.vel.y = 0.0;
            }
        }

        // player
        draw_rectangle(body.pos.x, body.pos.y, 50.0, 50.0, BLUE);

        // floor
        draw_rectangle(100.0, 500.0, 600.0, 20.0, GRAY);

        // velocity vector
        draw_line(
            body.pos.x + 25.0,
            body.pos.y + 25.0,
            body.pos.x + 25.0 + body.vel.x * 0.1,
            body.pos.y + 25.0 + body.vel.y * 0.1,
            3.0,
            RED,
        );
        if let Some(c) = &collision {
            draw_line(
                body.pos.x + 25.0,
                body.pos.y + 25.0,
                body.pos.x + 25.0 + c.normal.x * 100.0,
                body.pos.y + 25.0 + c.normal.y * 100.0,
                4.0,
                YELLOW,
            );
        }

        let vel_text = format!("vel : {:.1?}", body.vel);

        draw_text(&vel_text, 20.0, 30.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
