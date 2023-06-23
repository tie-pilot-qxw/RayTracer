use std::sync::Arc;

use crate::material::Material;
use crate::Point3;
use crate::Ray;
use crate::Vec3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
    front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = (r.direction() * outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }

    pub fn new() -> Self {
        Self {
            p: (Point3::zero()),
            normal: (Vec3::zero()),
            t: (0.),
            front_face: (false),
            mat_ptr: None,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
