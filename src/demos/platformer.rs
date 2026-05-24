use macroquad::prelude::*;

use crate::{Body, Vec2, advanced_collision, resolve_pos_static, resolve_velocity, update};

pub fn rect(pos: Vec2, w: f32, h: f32) -> Vec<Vec2> {
    vec![
        Vec2::new(pos.x, pos.y),
        Vec2::new(pos.x + w, pos.y),
        Vec2::new(pos.x + w, pos.y + h),
        Vec2::new(pos.x, pos.y + h),
    ]
}

pub async fn run() {
    let player_size = 30.0;
    let speed = 400.0;
    let jump_force = 500.0;

    let mut body = Body {
        pos: Vec2::new(100.0, 100.0),
        vel: Vec2::ZERO,
        use_gravity: true,
    };

    let platforms = vec![
        // floor
        rect(Vec2::new(50.0, 500.0), 750.0, 50.0),
        // floating platforms
        rect(Vec2::new(200.0, 400.0), 100.0, 20.0),
        rect(Vec2::new(400.0, 300.0), 150.0, 20.0),
        rect(Vec2::new(650.0, 200.0), 100.0, 20.0),
        // wall
        rect(Vec2::new(50.0, 100.0), 50.0, 400.0),
        rect(Vec2::new(750.0, 100.0), 50.0, 400.0),
    ];

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        let mut grounded = false;

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            body.vel.x = speed;
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            body.vel.x = -speed;
        } else {
            body.vel.x *= 0.8;
        }

        // apply the physics.
        update(&mut body, dt);
        let mut player = rect(body.pos, player_size, player_size);

        // collision : player, platform
        for platform in &platforms {
            if let Some(c) = advanced_collision(&player, platform) {
                resolve_pos_static(&mut body.pos, c.normal, c.depth);
                resolve_velocity(&mut body.vel, c.normal, 0.0);

                if c.normal.y < -0.5 {
                    grounded = true;
                    body.vel.y = 0.0;
                    body.pos.y -= 0.1;
                }

                player = rect(body.pos, player_size, player_size);
            }
        }

        if is_key_pressed(KeyCode::Space) && grounded {
            body.vel.y -= jump_force;
        }

        draw_rectangle(body.pos.x, body.pos.y, player_size, player_size, BLUE);

        for platform in &platforms {
            let start = platform[0];
            let width = platform[1].x - platform[0].x;
            let height = platform[3].y - platform[0].y;
            draw_rectangle(start.x, start.y, width, height, GRAY);
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
