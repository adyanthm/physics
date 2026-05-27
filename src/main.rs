use collision::demos::{demolition, platformer, sandbox, visualize};
use macroquad::prelude::*;
#[macroquad::main("Collision sandbox")]

async fn main() {
    let mut current_demo = 0;

    loop {
        if current_demo == 0 {
            clear_background(BLACK);
            draw_text("Collision Engine Sandbox", 100.0, 100.0, 40.0, WHITE);
            draw_text(
                "Press 1: Castle Demolition Demo",
                100.0,
                200.0,
                30.0,
                LIGHTGRAY,
            );
            draw_text("Press 2: Platformer Demo", 100.0, 250.0, 30.0, LIGHTGRAY);
            draw_text(
                "Press 3: Sandbox / Pile Demo",
                100.0,
                300.0,
                30.0,
                LIGHTGRAY,
            );
            draw_text(
                "Press 4: Falling Box (Gravity) Demo",
                100.0,
                350.0,
                30.0,
                LIGHTGRAY,
            );
            draw_text(
                "Press ESC inside a demo to return to this menu.",
                100.0,
                420.0,
                20.0,
                YELLOW,
            );

            if is_key_pressed(KeyCode::Key1) {
                current_demo = 1;
            }
            if is_key_pressed(KeyCode::Key2) {
                current_demo = 2;
            }
            if is_key_pressed(KeyCode::Key3) {
                current_demo = 3;
            }
            if is_key_pressed(KeyCode::Key4) {
                current_demo = 4;
            }
            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            next_frame().await;
        } else if current_demo == 1 {
            demolition::run().await;
            current_demo = 0;
            next_frame().await;
        } else if current_demo == 2 {
            platformer::run().await;
            current_demo = 0;
            next_frame().await;
        } else if current_demo == 3 {
            sandbox::run().await;
            current_demo = 0;
            next_frame().await;
        } else if current_demo == 4 {
            visualize::run().await;
            current_demo = 0;
            next_frame().await;
        }
    }
}
