use crate::{
    material::{texture::Texture, Isotropic},
    rtweekend::random_double_unit,
    Color3, Vec3,
};
use std::{
    f64::{consts::E, INFINITY},
    sync::Arc,
};

use super::{HitRecord, Hittable, Material};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Send + Sync>,
    phase_function: Arc<dyn Material + Sync + Send>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    #[allow(dead_code)]
    pub fn new_texture(
        boundary: Arc<dyn Hittable + Send + Sync>,
        d: f64,
        a: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new_texture(a)),
            neg_inv_density: (-1. / d),
        }
    }

    pub fn new_color(boundary: Arc<dyn Hittable + Send + Sync>, d: f64, c: Color3) -> Self {
        Self {
            boundary,
            phase_function: (Arc::new(Isotropic::new_color(c))),
            neg_inv_density: (-1. / d),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut super::aabb::AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }

    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut super::HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double_unit().log(E);

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1., 0., 0.); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat_ptr = Some(self.phase_function.clone());

        true
    }
}
