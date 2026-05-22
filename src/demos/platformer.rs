use macroquad::prelude::*;

use crate::{Body, Vec2, advanced_collision, resolve_pos_static, resolve_velocity, update};

pub fn rect(pos: Vec2, w: f32, h: f32) -> Vec<Vec2> {
    vec![
        (pos.0, pos.1),
        (pos.0 + w, pos.1),
        (pos.0 + w, pos.1 + h),
        (pos.0, pos.1 + h),
    ]
}

pub async fn run() {
    let player_size = 30.0;
    let speed = 400.0;
    let jump_force = 500.0;

    let mut body = Body {
        pos: (100.0, 100.0),
        vel: (0.0, 0.0),
        use_gravity: true,
    };

    let platforms = vec![
        // floor
        rect((50.0, 500.0), 750.0, 50.0),
        // floating platforms
        rect((200.0, 400.0), 100.0, 20.0),
        rect((400.0, 300.0), 150.0, 20.0),
        rect((650.0, 200.0), 100.0, 20.0),
        // wall
        rect((50.0, 100.0), 50.0, 400.0),
        rect((750.0, 100.0), 50.0, 400.0),
    ];

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        let mut grounded = false;

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            body.vel.0 = speed;
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            body.vel.0 = -speed;
        } else {
            body.vel.0 *= 0.8;
        }

        // apply the physics.
        update(&mut body, dt);
        let mut player = rect(body.pos, player_size, player_size);

        // collision : player, platform
        for platform in &platforms {
            let (hit, normal, depth) = advanced_collision(&player, platform);

            if hit {
                resolve_pos_static(&mut body.pos, normal, depth);

                let response_normal = (-normal.0, -normal.1);
                resolve_velocity(&mut body.vel, response_normal, 0.0);

                if response_normal.1 < -0.5 {
                    grounded = true;
                    body.vel.1 = 0.0;
                    body.pos.1 -= 0.1;
                }

                player = rect(body.pos, player_size, player_size);
            }
        }

        if is_key_pressed(KeyCode::Space) && grounded {
            body.vel.1 -= jump_force;
        }

        draw_rectangle(body.pos.0, body.pos.1, player_size, player_size, BLUE);

        for platform in &platforms {
            let (x, y) = platform[0];
            let size_x = platform[1].0 - platform[0].0;
            let size_y = platform[3].1 - platform[0].1;
            draw_rectangle(x, y, size_x, size_y, GRAY);
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
