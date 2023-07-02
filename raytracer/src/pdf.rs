use std::f64::consts::PI;

use crate::{onb::Onb, rtweekend::random_cosine_direction, Vec3};

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        let uvw = Onb::build_from_w(*w);
        Self { uvw }
    }
}

impl Pdf for CosinePdf {
    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&random_cosine_direction())
    }

    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.unit() * self.uvw.w();
        if cosine <= 0_f64 {
            0.
        } else {
            cosine / PI
        }
    }
}
