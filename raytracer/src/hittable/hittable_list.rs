use std::sync::Arc;

use crate::{
    hittable::aabb::AABB,
    hittable::{HitRecord, Hittable},
    Point3,
};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }

    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut temp_rec = HitRecord::new();
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
                temp_rec = HitRecord::new();
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::new(Point3::zero(), Point3::zero());
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                AABB::surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }

        true
    }
}
