use std::f64::consts::PI;

pub fn degrees_to_radius(degrees: f64) -> f64 {
    degrees * PI / 180.
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
