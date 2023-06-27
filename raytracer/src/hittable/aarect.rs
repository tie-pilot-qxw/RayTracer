use std::sync::Arc;

use crate::{Point3, Vec3};

use super::{aabb::AABB, HitRecord, Hittable, Material};

pub struct XyRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64, // z
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XyRect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }

    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0., 0., 1.);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.p = r.at(t);
        true
    }
}

pub struct XzRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64, // y
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XzRect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }

    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0., 1., 0.);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.p = r.at(t);
        true
    }
}

pub struct YzRect {
    mp: Arc<dyn Material>,
    z0: f64,
    z1: f64,
    y0: f64,
    y1: f64,
    k: f64, // x
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Arc<dyn Material>) -> Self {
        Self {
            mp,
            z0,
            z1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for YzRect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }

    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return false;
        }

        let z = r.origin().z() + t * r.direction().z();
        let y = r.origin().y() + t * r.direction().y();
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(1., 0., 0.);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.p = r.at(t);
        true
    }
}
