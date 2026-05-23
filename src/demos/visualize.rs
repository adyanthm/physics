use macroquad::prelude::*;

use crate::{Body, Vec2, advanced_collision, resolve_pos_static, resolve_velocity, update};

pub fn rect(pos: Vec2, size: f32) -> Vec<Vec2> {
    vec![
        (pos.0, pos.1),
        (pos.0 + size, pos.1),
        (pos.0 + size, pos.1 + size),
        (pos.0, pos.1 + size),
    ]
}

pub async fn run() {
    let mut body = Body {
        pos: (350.0, 50.0),
        vel: (0.0, 0.0),
        use_gravity: true,
    };

    let floor = [
        (100.0, 500.0),
        (700.0, 500.0),
        (700.0, 520.0),
        (100.0, 520.0),
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

            if c.normal.1 < -0.5 {
                body.pos.1 -= 0.1;
                body.vel.1 = 0.0;
            }
        }

        // player
        draw_rectangle(body.pos.0, body.pos.1, 50.0, 50.0, BLUE);

        // floor
        draw_rectangle(100.0, 500.0, 600.0, 20.0, GRAY);

        // velocity vector
        draw_line(
            body.pos.0 + 25.0,
            body.pos.1 + 25.0,
            body.pos.0 + 25.0 + body.vel.0 * 0.1,
            body.pos.1 + 25.0 + body.vel.1 * 0.1,
            3.0,
            RED,
        );
        if let Some(c) = &collision {
            draw_line(
                body.pos.0 + 25.0,
                body.pos.1 + 25.0,
                body.pos.0 + 25.0 + c.normal.0 * 100.0,
                body.pos.1 + 25.0 + c.normal.1 * 100.0,
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
