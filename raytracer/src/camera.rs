use crate::{ray::Ray, rtweekend::degrees_to_radius, Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64, // vertical field-of-view in degrees
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radius(vfov);
        let h = (theta / 2.).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(vup, w);
        let v = Vec3::cross(w, u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Self {
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner: lookfrom - horizontal / 2.0 - vertical / 2.0 - w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
