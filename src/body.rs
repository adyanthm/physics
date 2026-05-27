use crate::Vec2;

#[derive(Clone, Copy, PartialEq)]
pub enum BodyType {
    Dynamic,
    Static,
}

pub struct RigidBody {
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f32,
    pub angular_vel: f32,
    pub bias_vel: Vec2,
    pub bias_angular_vel: f32,
    pub awake: bool,
    pub sleep_timer: f32,
    pub force: Vec2,
    pub mass: f32,
    pub inv_mass: f32,
    pub inertia: f32,
    pub inv_inertia: f32,
    pub body_type: BodyType,
    pub use_gravity: bool,
    pub restitution: f32,
    pub friction: f32,
    pub vertices: Vec<Vec2>,
}

impl RigidBody {
    pub fn new_dynamic(pos: Vec2, vertices: Vec<Vec2>) -> Self {
        Self::new_dynamic_with_density(pos, vertices, 1.0)
    }

    pub fn new_dynamic_with_density(pos: Vec2, mut vertices: Vec<Vec2>, density: f32) -> Self {
        center_vertices(&mut vertices);
        let area = compute_polygon_area(&vertices);
        let mass = density * area;
        let inertia = compute_polygon_inertia(&vertices, mass);
        Self {
            pos,
            vel: Vec2::ZERO,
            angle: 0.0,
            angular_vel: 0.0,
            bias_vel: Vec2::ZERO,
            bias_angular_vel: 0.0,
            awake: true,
            sleep_timer: 0.0,
            force: Vec2::ZERO,
            mass,
            inv_mass: if mass > 0.0 { 1.0 / mass } else { 0.0 },
            inertia,
            inv_inertia: if inertia > 0.0 { 1.0 / inertia } else { 0.0 },
            body_type: BodyType::Dynamic,
            use_gravity: true,
            restitution: 0.0,
            friction: 0.3,
            vertices,
        }
    }

    pub fn new_static(pos: Vec2, mut vertices: Vec<Vec2>) -> Self {
        center_vertices(&mut vertices);
        Self {
            pos,
            vel: Vec2::ZERO,
            angle: 0.0,
            angular_vel: 0.0,
            bias_vel: Vec2::ZERO,
            bias_angular_vel: 0.0,
            awake: false,
            sleep_timer: 0.0,
            force: Vec2::ZERO,
            mass: 0.0,
            inv_mass: 0.0,
            inertia: 0.0,
            inv_inertia: 0.0,
            body_type: BodyType::Static,
            use_gravity: false,
            restitution: 0.0,
            friction: 0.5,
            vertices,
        }
    }

    pub fn world_vertices(&self) -> Vec<Vec2> {
        let cos = self.angle.cos();
        let sin = self.angle.sin();
        self.vertices
            .iter()
            .map(|&v| {
                Vec2::new(
                    v.x * cos - v.y * sin + self.pos.x,
                    v.x * sin + v.y * cos + self.pos.y,
                )
            })
            .collect()
    }
}

pub fn rect_vertices(w: f32, h: f32) -> Vec<Vec2> {
    let hw = w / 2.0;
    let hh = h / 2.0;
    vec![
        Vec2::new(-hw, -hh),
        Vec2::new(hw, -hh),
        Vec2::new(hw, hh),
        Vec2::new(-hw, hh),
    ]
}

pub fn compute_centroid(vertices: &[Vec2]) -> Vec2 {
    if vertices.is_empty() {
        return Vec2::ZERO;
    }
    let mut cx = 0.0_f32;
    let mut cy = 0.0_f32;
    let mut area_sum = 0.0_f32;
    let n = vertices.len();

    for i in 0..n {
        let v0 = vertices[i];
        let v1 = vertices[(i + 1) % n];
        let cross = v0.x * v1.y - v1.x * v0.y;
        area_sum += cross;
        cx += (v0.x + v1.x) * cross;
        cy += (v0.y + v1.y) * cross;
    }

    let area6 = area_sum * 3.0;
    if area6.abs() < f32::EPSILON {
        let mut total = Vec2::ZERO;
        for &v in vertices {
            total += v;
        }
        return total / n as f32;
    }
    Vec2::new(cx / area6, cy / area6)
}

pub fn center_vertices(vertices: &mut Vec<Vec2>) {
    let c = compute_centroid(vertices);
    for v in vertices.iter_mut() {
        *v = *v - c;
    }
}

pub fn compute_polygon_area(vertices: &[Vec2]) -> f32 {
    if vertices.len() < 3 {
        return 0.0;
    }
    let mut area = 0.0_f32;
    let n = vertices.len();
    for i in 0..n {
        let v0 = vertices[i];
        let v1 = vertices[(i + 1) % n];
        area += v0.x * v1.y - v1.x * v0.y;
    }
    (area / 2.0).abs()
}

pub fn compute_polygon_inertia(vertices: &[Vec2], mass: f32) -> f32 {
    if vertices.len() < 3 || mass <= 0.0 {
        return 0.0;
    }
    let mut numerator = 0.0_f32;
    let mut denominator = 0.0_f32;
    let n = vertices.len();

    for i in 0..n {
        let v0 = vertices[i];
        let v1 = vertices[(i + 1) % n];
        let cross = (v0.cross(v1)).abs();
        numerator += cross * (v0.dot(v0) + v0.dot(v1) + v1.dot(v1));
        denominator += cross;
    }

    if denominator < f32::EPSILON {
        return 0.0;
    }

    mass * numerator / (6.0 * denominator)
}
