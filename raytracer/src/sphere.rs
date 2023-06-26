use std::f64::consts::PI;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::Point3;
use crate::Vec3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }

    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        *u = phi / (2. * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().squared_length();
        let half_b = oc * r.direction();
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Self::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = Some(self.mat_ptr.clone());
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}

#[test]
fn test_get_uv() {
    let mut u = 0.;
    let mut v = 0.;
    Sphere::get_sphere_uv(&Point3::new(0., 0., -1.), &mut u, &mut v);
    assert_eq!(u, 0.75);
    assert_eq!(v, 0.5);
}
