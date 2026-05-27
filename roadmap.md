1. Swept / Continuous Collision Detection (Completed)

* Prevent fast objects from tunneling through walls
* Detect collision over time instead of per-frame overlap
* Time-of-impact calculations
* Stable high-speed collisions

2. Contact Points (Completed)

* Calculate exact collision points
* More accurate collision response
* Needed for realistic physics
* Foundation for friction and stable stacking

3. Impulse Resolution (Completed)

* Apply forces after collision
* Realistic bouncing
* Mass and momentum transfer
* Elastic and inelastic collisions

4. Spatial Partitioning

* Optimize many-object collision checks
* Uniform grids
* Quadtrees
* Bounding Volume Hierarchies (BVH)
* Avoid O(n²) collision scaling

5. Better Geometry Types (Completed)

* Replace tuple vectors with proper Vec2 struct
* Add Circle, Polygon, AABB structs
* Cleaner APIs and methods
* Operator overloading (+, -, *)

6. Transform System (Completed)

* Engine architecture updated with basic `RigidBody` and `PhysicsWorld`
* Local-space vs world-space polygons (Implemented for RigidBody)
* Translation, scaling, rotation systems (Rotation pending)
* Reusable shapes with transforms

7. Rotation Support (Completed)

* Rotating polygons
* Rotation matrices
* Angular velocity
* Rotated collision handling

8. Concave Polygon Support

* Convex decomposition
* Triangulation
* Arbitrary polygon collision
* Ear clipping algorithms

9. GJK + EPA Algorithms

* Advanced convex collision detection
* Faster/more scalable than SAT
* Support for many shape types
* Penetration depth calculation with EPA

10. Constraints and Joints

* Springs
* Hinges
* Ropes
* Distance joints
* Full rigid-body physics systems
