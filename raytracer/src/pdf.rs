pub mod onb;

use std::{f64::consts::PI, sync::Arc};

use crate::{hittable::Hittable, rtweekend::random_double_unit, Point3, Vec3};

use self::onb::Onb;

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double_unit();
    let r2 = random_double_unit();
    let z = (1. - r2).sqrt();

    let phi = 2. * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random_double_unit();
    let r2 = random_double_unit();
    let z = 1. + r2 * ((1. - radius * radius / distance_squared).sqrt() - 1.);

    let phi = 2. * PI * r1;
    let x = phi.cos() * (1. - z * z).sqrt();
    let y = phi.sin() * (1. - z * z).sqrt();

    Vec3::new(x, y, z)
}

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

pub struct MixturePdf {
    p: [Arc<dyn Pdf + Send + Sync>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf + Send + Sync>, p1: Arc<dyn Pdf + Send + Sync>) -> Self {
        Self { p: ([p0, p1]) }
    }
}

impl Pdf for MixturePdf {
    fn generate(&self) -> Vec3 {
        if random_double_unit() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }

    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
}
