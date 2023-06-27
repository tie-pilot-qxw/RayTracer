use std::cmp::Ordering;
use std::sync::Arc;

use crate::hittable::aabb::AABB;
use crate::hittable::sphere::Sphere;
use crate::hittable::HitRecord;
use crate::material::Dielectric;
use crate::ray::Ray;
use crate::rtweekend::random_int;
use crate::Point3;
use crate::{hittable::hittable_list::HittableList, hittable::Hittable};

#[allow(clippy::upper_case_acronyms)]
pub struct BVH {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    tree_box: AABB,
}

impl BVH {
    pub fn new(list: &HittableList, time0: f64, time1: f64) -> Self {
        Self::new_p(list.objects.as_slice(), time0, time1)
    }

    fn new_p(src_objects: &[Arc<dyn Hittable>], time0: f64, time1: f64) -> Self {
        let mut objects = src_objects.to_vec();

        let axis = random_int(0, 2);
        let comparator = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };

        let object_span = objects.len();
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if object_span == 0 {
            left = Arc::new(Sphere::new(
                Point3::zero(),
                0.,
                Arc::new(Dielectric::new(1.)),
            ));
            right = left.clone();
        } else if object_span == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if object_span == 2 {
            if comparator(&objects[0], &objects[1], time0, time1) == Some(Ordering::Less) {
                left = objects[0].clone();
                right = objects[1].clone();
            } else {
                left = objects[1].clone();
                right = objects[0].clone();
            }
        } else {
            objects.sort_by(|a, b| comparator(a, b, time0, time1).unwrap());

            let mid = object_span / 2;
            left = Arc::new(BVH::new_p(&objects[0..mid], time0, time1));
            right = Arc::new(BVH::new_p(&objects[mid..object_span], time0, time1));
        }

        let mut box_left = AABB::new(Point3::zero(), Point3::zero());
        let mut box_right = AABB::new(Point3::zero(), Point3::zero());

        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            panic!("No bounding box in bvh_node constructor!");
        }

        let tree_box = AABB::surrounding_box(&box_left, &box_right);
        Self {
            left,
            right,
            tree_box,
        }
    }

    fn box_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        axis: usize,
        time0: f64,
        time1: f64,
    ) -> Option<Ordering> {
        let mut box_a = AABB::new(Point3::zero(), Point3::zero());
        let mut box_b = AABB::new(Point3::zero(), Point3::zero());

        if !a.bounding_box(time0, time1, &mut box_a) || !b.bounding_box(time0, time1, &mut box_b) {
            panic!("No bounding box in bvh_node constructor!");
        }

        box_a.min()[axis].partial_cmp(&box_b.min()[axis])
    }

    fn box_x_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        time0: f64,
        time1: f64,
    ) -> Option<Ordering> {
        Self::box_compare(a, b, 0, time0, time1)
    }

    fn box_y_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        time0: f64,
        time1: f64,
    ) -> Option<Ordering> {
        Self::box_compare(a, b, 1, time0, time1)
    }

    fn box_z_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        time0: f64,
        time1: f64,
    ) -> Option<Ordering> {
        Self::box_compare(a, b, 2, time0, time1)
    }
}

impl Hittable for BVH {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.tree_box;
        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.tree_box.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }
}
