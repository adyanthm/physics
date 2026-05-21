1. Collision Resolution

* Use collision normal + penetration depth
* Push objects apart after collision
* Prevent objects from overlapping
* Basic solid walls and object separation

2. Velocity and Physics

* Add velocity vectors to objects
* Movement and acceleration
* Gravity and jumping
* Basic bouncing and momentum

3. Swept / Continuous Collision Detection

* Prevent fast objects from tunneling through walls
* Detect collision over time instead of per-frame overlap
* Time-of-impact calculations
* Stable high-speed collisions

4. Contact Points

* Calculate exact collision points
* More accurate collision response
* Needed for realistic physics
* Foundation for friction and stable stacking

5. Impulse Resolution

* Apply forces after collision
* Realistic bouncing
* Mass and momentum transfer
* Elastic and inelastic collisions

6. Spatial Partitioning

* Optimize many-object collision checks
* Uniform grids
* Quadtrees
* Bounding Volume Hierarchies (BVH)
* Avoid O(n²) collision scaling

7. Better Geometry Types

* Replace tuple vectors with proper Vec2 struct
* Add Circle, Polygon, AABB structs
* Cleaner APIs and methods
* Operator overloading (+, -, *)

8. Transform System

* Local-space vs world-space polygons
* Translation, scaling, rotation systems
* Reusable shapes with transforms
* Better engine architecture

9. Rotation Support

* Rotating polygons
* Rotation matrices
* Angular velocity
* Rotated collision handling

10. Concave Polygon Support

* Convex decomposition
* Triangulation
* Arbitrary polygon collision
* Ear clipping algorithms

11. GJK + EPA Algorithms

* Advanced convex collision detection
* Faster/more scalable than SAT
* Support for many shape types
* Penetration depth calculation with EPA

12. Constraints and Joints

* Springs
* Hinges
* Ropes
* Distance joints
* Full rigid-body physics systems
