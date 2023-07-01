use std::f64::consts::PI;

use crate::Vec3;

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

pub fn random_double_unit() -> f64 {
    rand::random::<f64>()
}

pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double_unit()
}

pub fn random_int(min: isize, max: isize) -> isize {
    random_double(min as f64, max as f64 + 1.) as isize
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double_unit();
    let r2 = random_double_unit();
    let z = (1. - r2).sqrt();

    let phi = 2. * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}
