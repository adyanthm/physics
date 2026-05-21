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
