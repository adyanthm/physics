# Contributing

Thanks for your interest. This section explains how the codebase is structured and how the core algorithm works.

## Source tree

```
src/
  lib.rs       — module declarations and public re-exports
  vectors.rs   — Vec2 type alias, dot product, normalize, magnitude
  aabb.rs      — axis-aligned bounding box computation and overlap check
  sat.rs       — separating axis theorem (convex), MTV calculation
  concave.rs   — concave polygon collision (vertex containment + edge intersection)
  point.rs     — point-in-polygon via ray casting
  velocity.rs  — body physics, gravity, and velocity resolution
  demos/       — interactive examples (platformer, falling box)
  main.rs      — graphical sandbox entry point
```

## How SAT works

The Separating Axis Theorem says: two convex shapes do **not** collide if and only if there exists an axis where their projections don't overlap. If no such axis exists, they are colliding.

The axes we test are the **edge normals** of both polygons. For each edge, we compute the perpendicular (normal) vector. Then for each normal:

1. Project every vertex of polygon A onto the axis. Record the min and max.
2. Project every vertex of polygon B onto the axis. Record the min and max.
3. Check if the two intervals `[minA, maxA]` and `[minB, maxB]` overlap.

If any axis has **no overlap**, the shapes are separated — return false immediately. If all axes overlap, the shapes are colliding.

To get the **Minimum Translation Vector** (MTV), we track which axis had the smallest overlap. That axis direction and overlap depth tell you exactly how far to push the shapes apart to resolve the collision. `advanced_collision` in `sat.rs` returns this as `(bool, Vec2, f32)`.

## AABB broadphase

Before running SAT (which loops over every edge of both polygons), we first check if the axis-aligned bounding boxes overlap. What AABB does is really simple. It just draws a rectangle over polygon and sees if the rectangles of different polygons overlap. This calculation is really cheap as it only involves min/max comparison on x and y. If the boxes don't overlap, the shapes can't possibly collide, and we skip SAT entirely. This matters a lot when you have many objects.

## Concave polygons

SAT only works on convex shapes. For concave polygons, `concave.rs` uses three checks:

1. Is any vertex of polygon A inside polygon B? (ray casting)
2. Is any vertex of polygon B inside polygon A? (ray casting)
3. Do any edges of A cross any edges of B? (segment intersection)

If any of those is true, the shapes collide.

## Making changes

- Run `cargo test` before submitting anything.
- Keep functions small and focused.
- If you add a new module, re-export its public API from `lib.rs`.
