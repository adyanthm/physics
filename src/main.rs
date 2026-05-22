use collision::demos::{platformer, visualize};
use macroquad::prelude::*;
#[macroquad::main("Collision sandbox")]

async fn main() {
    // 0 = menu, 1 = visualize (gravity), 2 = platformer
    let mut current_demo = 0;

    loop {
        if current_demo == 0 {
            clear_background(BLACK);
            draw_text("Collision Engine Sandbox", 100.0, 100.0, 40.0, WHITE);
            draw_text(
                "Press 1: Falling Box (Gravity) Demo",
                100.0,
                200.0,
                30.0,
                LIGHTGRAY,
            );
            draw_text("Press 2: Platformer Demo", 100.0, 250.0, 30.0, LIGHTGRAY);
            draw_text(
                "Press ESC inside a demo to return to this menu.",
                100.0,
                350.0,
                20.0,
                YELLOW,
            );

            if is_key_pressed(KeyCode::Key1) {
                current_demo = 1; // visualize.rs
            }
            if is_key_pressed(KeyCode::Key2) {
                current_demo = 2; // platformer.rs
            }
            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            next_frame().await;
        } else if current_demo == 1 {
            visualize::run().await;
            current_demo = 0; // return to menu once done
            next_frame().await;
            // ^ Clears the Escape key press from the input buffer
            // so it doesn't accidentally trigger the main menu exit.
        } else if current_demo == 2 {
            platformer::run().await;
            current_demo = 0; // return to menu once done
            next_frame().await;
            // ^ Clears the Escape key press from the input buffer
            // so it doesn't accidentally trigger the main menu exit.
        }
    }
}
