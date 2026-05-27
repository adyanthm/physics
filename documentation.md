# Physics Engine Documentation

This document provides examples of how to use the physics engine, ordered from the most common tasks to more advanced/internal use cases. The engine is structured around a `PhysicsWorld` that manages the simulation, and `RigidBody` instances that represent physical objects. The API design is inspired by Box2D.

---

## Common Use Cases

### 1. Setting up the World and Stepping the Simulation
The first step in using the physics engine is to initialize a `PhysicsWorld` and step it inside your game loop.

```rust
use collision::world::PhysicsWorld;

// Create a new physics world
let mut world = PhysicsWorld::new();

// In your game loop:
let dt = 0.016; // Time step (delta time)
world.step(dt);
```

### 2. Creating Static Bodies (Floors, Walls)
Static bodies don't move and aren't affected by gravity, but dynamic bodies will collide with them. Use them for your level geometry.

```rust
use collision::body::{RigidBody, rect_vertices};
use collision::Vec2;

// Create a static floor
let floor_handle = world.add_body(RigidBody::new_static(
    Vec2::new(100.0, 500.0), // Position (x, y)
    rect_vertices(600.0, 20.0), // Width 600, Height 20
));
```

### 3. Creating Dynamic Bodies (Player, Crates)
Dynamic bodies are affected by gravity and resolve collisions by moving and bouncing.

```rust
use collision::body::{RigidBody, rect_vertices};
use collision::Vec2;

let player_handle = world.add_body(RigidBody::new_dynamic(
    Vec2::new(300.0, 100.0),
    rect_vertices(50.0, 50.0),
));
```

### 4. Moving Bodies and Jumping
To move a body, access it through the world's `bodies` array and modify its velocity.

```rust
// Access the body
let player = &mut world.bodies[player_handle];

// Move right by setting horizontal velocity
player.vel.x = 400.0;

// Apply friction/drag if no key is pressed
// player.vel.x *= 0.9;

// Jump (check if grounded first)
if world.is_grounded(player_handle) {
    player.vel.y = -500.0; // Negative Y is up
}
```

### 5. Modifying Physical Properties (Restitution, Gravity)
You can customize physics behavior per object, such as bounciness (`restitution`) or turning off gravity.

```rust
let mut bouncy_box = RigidBody::new_dynamic(
    Vec2::new(200.0, 100.0),
    rect_vertices(40.0, 40.0),
);

// Set bounciness (0.0 is no bounce, 1.0 is perfectly elastic)
bouncy_box.restitution = 0.7;

// Disable gravity for this specific object
bouncy_box.use_gravity = false;

world.add_body(bouncy_box);
```

---

## Less Frequent Use Cases

### 6. Using Custom Polygon Shapes
You aren't limited to rectangles. You can pass any list of vertices (in local-space, relative to `Vec2::ZERO`) to create custom convex shapes.

```rust
let triangle_shape = vec![
    Vec2::new(0.0, 0.0),
    Vec2::new(50.0, 0.0),
    Vec2::new(25.0, 50.0),
];

world.add_body(RigidBody::new_dynamic(
    Vec2::new(100.0, 100.0),
    triangle_shape,
));
```

### 7. Reading Collision Contacts
If you need to know exactly what is colliding with what (e.g., to trigger a sound effect, take damage, or spawn particles), you can iterate through the `contacts` array after `world.step()`.

```rust
for contact in &world.contacts {
    if contact.body_a == player_handle || contact.body_b == player_handle {
        println!("Player collided with normal: ({}, {})", contact.normal.x, contact.normal.y);
        println!("Penetration depth: {}", contact.depth);
    }
}
```

### 8. Direct Raw Collision Algorithms
You can use the lower-level collision functions directly if you want to perform geometric overlap tests outside of the `PhysicsWorld` simulation.

```rust
use collision::sat::advanced_collision;
use collision::aabb::aabb_overlap;
use collision::point::point_in_polygon;

// Vertices must be in world-space for direct checks
let shape_a = vec![/* ... */];
let shape_b = vec![/* ... */];

// AABB overlap check (fast broadphase)
if aabb_overlap(&shape_a, &shape_b) {
    
    // SAT precise collision check (slower narrowphase)
    if let Some(collision) = advanced_collision(&shape_a, &shape_b) {
        println!("Colliding! MTV Normal: {:?}, Depth: {}", collision.normal, collision.depth);
    }
}

// Point-in-polygon check (useful for mouse clicking objects)
let mouse_pos = Vec2::new(150.0, 150.0);
if point_in_polygon(mouse_pos, &shape_a) {
    println!("Mouse is inside the shape!");
}
```

### 9. High-Velocity Objects and CCD
The engine automatically handles extremely fast objects using Continuous Collision Detection (CCD). By default, if a body moves faster than 575 pixels/second (enough to tunnel through a small shape in a 60fps frame), it will use swept-SAT to prevent it from ghosting through walls. No setup is required.

### 10. Body Sleeping
Objects that come to rest will automatically "go to sleep" after 0.5 seconds. Sleeping bodies skip physics processing, which improves performance and mathematically eliminates micro-jitter in stacked piles. They wake up instantly when hit by an active object.

```rust
// You can manually wake up a sleeping body
world.bodies[box_id].awake = true;
world.bodies[box_id].sleep_timer = 0.0;
```
