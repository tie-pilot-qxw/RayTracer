use std::{f64::consts::PI, sync::Arc};

use crate::{hittable::Hittable, onb::Onb, rtweekend::random_cosine_direction, Point3, Vec3};

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    #[allow(dead_code)]
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

pub struct HittablePdf {
    o: Point3,
    ptr: Arc<dyn Hittable + Send + Sync>,
}

impl HittablePdf {
    pub fn new(p: Arc<dyn Hittable + Send + Sync>, origin: &Point3) -> Self {
        Self { o: *origin, ptr: p }
    }
}

impl Pdf for HittablePdf {
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }

    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }
}
