pub type Vec2 = (f32, f32);

pub fn dot(a: Vec2, b: Vec2) -> f32 {
    a.0 * b.0 + a.1 * b.1
}

pub fn magnitude(x: f32, y: f32) -> f32 {
    (x.powi(2) + y.powi(2)).sqrt()
}

pub fn normalize(x: f32, y: f32) -> (f32, f32) {
    let length = magnitude(x, y);
    if length == 0.0 {
        (0.0, 0.0)
    } else {
        (x / length, y / length)
    }
}

pub fn add(a: Vec2, b: Vec2) -> Vec2 {
    (a.0 + b.0, a.1 + b.1)
}

pub fn sub(a: Vec2, b: Vec2) -> Vec2 {
    (a.0 - b.0, a.1 - b.1)
}

pub fn scale(v: Vec2, factor: f32) -> Vec2 {
    (v.0 * factor, v.1 * factor)
}
