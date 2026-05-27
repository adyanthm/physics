use crate::body::{rect_vertices, RigidBody};
use crate::world::PhysicsWorld;
use crate::Vec2;
use macroquad::prelude::*;

struct RenderBody {
    handle: usize,
    color: Color,
}

#[derive(PartialEq)]
enum FiringState {
    Idle,
    Aiming,
    Fired,
}

fn regular_polygon(radius: f32, sides: usize, angle_offset: f32) -> Vec<Vec2> {
    let mut vertices = Vec::new();
    let angle_step = std::f32::consts::PI * 2.0 / sides as f32;
    for i in 0..sides {
        let angle = i as f32 * angle_step + angle_offset;
        vertices.push(Vec2::new(angle.cos() * radius, angle.sin() * radius));
    }
    vertices
}

fn reset_level(world: &mut PhysicsWorld, render_bodies: &mut Vec<RenderBody>) {
    world.bodies.clear();
    world.contacts.clear();
    render_bodies.clear();

    let floor = RigidBody::new_static(Vec2::new(400.0, 550.0), rect_vertices(800.0, 50.0));
    let floor_id = world.add_body(floor);
    render_bodies.push(RenderBody {
        handle: floor_id,
        color: GRAY,
    });

    let mut add_box = |x: f32, y: f32, w: f32, h: f32, color: Color| {
        let mut body = RigidBody::new_dynamic(Vec2::new(x, y), rect_vertices(w, h));
        body.restitution = 0.0;
        body.friction = 0.9;
        let handle = world.add_body(body);
        render_bodies.push(RenderBody { handle, color });
    };

    add_box(600.0, 485.0, 50.0, 80.0, BLUE);
    add_box(700.0, 485.0, 50.0, 80.0, BLUE);
    add_box(650.0, 430.0, 160.0, 30.0, GREEN);
    add_box(650.0, 500.0, 40.0, 50.0, YELLOW);

    add_box(605.0, 375.0, 50.0, 80.0, BLUE);
    add_box(695.0, 375.0, 50.0, 80.0, BLUE);
    add_box(650.0, 320.0, 120.0, 30.0, GREEN);
    add_box(650.0, 390.0, 30.0, 50.0, YELLOW);

    let mut roof = RigidBody::new_dynamic(
        Vec2::new(650.0, 280.0),
        regular_polygon(50.0, 3, -std::f32::consts::PI / 2.0),
    );
    roof.restitution = 0.1;
    let roof_id = world.add_body(roof);
    render_bodies.push(RenderBody {
        handle: roof_id,
        color: RED,
    });
}

pub async fn run() {
    let mut world = PhysicsWorld::new();
    let mut render_bodies = Vec::new();

    reset_level(&mut world, &mut render_bodies);

    let anchor = Vec2::new(150.0, 350.0);
    let mut state = FiringState::Idle;
    let mut drag_pos = anchor;

    loop {
        clear_background(color_u8!(20, 20, 25, 255));
        let dt = 1.0 / 60.0;

        if is_key_pressed(KeyCode::R) {
            reset_level(&mut world, &mut render_bodies);
            state = FiringState::Idle;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let mpos = mouse_position();
        let mouse_vec = Vec2::new(mpos.0, mpos.1);

        if is_mouse_button_pressed(MouseButton::Left) {
            let dist_to_anchor = (mouse_vec - anchor).magnitude();
            if dist_to_anchor < 80.0 {
                state = FiringState::Aiming;
            }
        }

        if state == FiringState::Aiming {
            drag_pos = mouse_vec;

            let drag_vec = drag_pos - anchor;
            if drag_vec.magnitude() > 100.0 {
                drag_pos = anchor + drag_vec.normalize() * 100.0;
            }

            if is_mouse_button_released(MouseButton::Left) {
                state = FiringState::Fired;

                let pull_vector = anchor - drag_pos;
                let velocity = pull_vector * 15.0;

                let mut cannonball =
                    RigidBody::new_dynamic(anchor, regular_polygon(15.0, 16, 0.0));
                cannonball.inv_mass *= 0.1;
                cannonball.inv_inertia *= 0.1;
                cannonball.restitution = 0.6;
                cannonball.vel = velocity;

                let cb_id = world.add_body(cannonball);
                render_bodies.push(RenderBody {
                    handle: cb_id,
                    color: DARKGRAY,
                });
            }
        }

        draw_circle(anchor.x, anchor.y, 8.0, BROWN);
        if state == FiringState::Aiming {
            draw_line(anchor.x, anchor.y, drag_pos.x, drag_pos.y, 5.0, BROWN);
            draw_circle(drag_pos.x, drag_pos.y, 15.0, DARKGRAY);

            let pull = anchor - drag_pos;
            let vel = pull * 15.0;
            let mut proj_p = drag_pos;
            let mut proj_v = vel;
            for _ in 0..30 {
                let next_p = proj_p + proj_v * dt;
                draw_line(
                    proj_p.x,
                    proj_p.y,
                    next_p.x,
                    next_p.y,
                    2.0,
                    color_u8!(255, 255, 255, 100),
                );
                proj_p = next_p;
                proj_v.y += world.gravity * dt;
            }
        } else {
            draw_circle(anchor.x, anchor.y, 15.0, DARKGRAY);
        }

        world.step(dt);

        for rb in &render_bodies {
            let body = &world.bodies[rb.handle];
            let verts = body.world_vertices();

            for i in 0..verts.len() {
                let p1 = verts[i];
                let p2 = verts[(i + 1) % verts.len()];
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, rb.color);
            }

            if body.awake && body.body_type == crate::body::BodyType::Dynamic {
                draw_circle(body.pos.x, body.pos.y, 2.0, WHITE);
            } else if !body.awake {
                draw_circle(body.pos.x, body.pos.y, 2.0, BLUE);
            }
        }

        draw_text("Castle Demolition Demo", 20.0, 30.0, 30.0, WHITE);
        draw_text("Click and drag the ball on the left to shoot!", 20.0, 60.0, 20.0, WHITE);
        draw_text("Press 'R' to reset", 20.0, 85.0, 20.0, WHITE);

        next_frame().await;
    }
}
