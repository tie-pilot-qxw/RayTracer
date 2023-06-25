use std::mem::swap;

use crate::{Point3, ray::Ray};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Self { minimum, maximum }
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.minimum[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.maximum[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0_f64 {
                swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let small = Point3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box1.min().z().min(box1.min().z())
        );
        let big = Point3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box1.max().z().max(box1.max().z())
        );
        Self { minimum: (small), maximum: (big) }
    }
}