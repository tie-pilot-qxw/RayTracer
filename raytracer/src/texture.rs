use std::sync::Arc;

use crate::{Color3, Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3;
}

pub struct SolidColor {
    color_value: Color3,
}

impl SolidColor {
    pub fn new(color_value: Color3) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color3 {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn new_solid(c1: Color3, c2: Color3) -> Self {
        Self {
            odd: (Arc::new(SolidColor::new(c1))),
            even: (Arc::new(SolidColor::new(c2))),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color3 {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
