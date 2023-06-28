mod aabb;
pub mod aarect;
pub mod boxes;
pub mod bvh;
pub mod constant_medium;
pub mod hittable_list;
pub mod moving_sphere;
pub mod sphere;

use std::f64::INFINITY;
use std::sync::Arc;

pub use crate::material::Material;
use crate::rtweekend::degrees_to_radius;
use crate::Point3;
use crate::Ray;
use crate::Vec3;
use aabb::AABB;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub mat_ptr: Option<Arc<dyn Material + Send + Sync>>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
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
            u: (0.),
            v: (0.),
            front_face: (false),
            mat_ptr: None,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}

pub struct Translate {
    ptr: Arc<dyn Hittable + Send + Sync>,
    offset: Vec3,
}

impl Translate {
    pub fn new(ptr: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = AABB::new(
            output_box.min() + self.offset,
            output_box.max() + self.offset,
        );

        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        rec.set_face_normal(&moved_r, rec.normal);

        true
    }
}

pub struct RotateY {
    ptr: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}

impl RotateY {
    pub fn new(ptr: Arc<dyn Hittable + Send + Sync>, angle: f64) -> Self {
        let radians = degrees_to_radius(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = AABB::new(Vec3::zero(), Vec3::zero());
        let hasbox = ptr.bounding_box(0., 1., &mut bbox);

        let mut min = INFINITY * Point3::ones();
        let mut max = (-INFINITY) * Point3::ones();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let newx = cos_theta * x + sin_theta * x;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox: AABB::new(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, normal);

        true
    }
}
