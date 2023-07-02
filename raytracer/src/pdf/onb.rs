use std::ops::{Index, IndexMut};

use crate::Vec3;

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    #[allow(dead_code)]
    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }

    pub fn local_vec(&self, a: &Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }

    pub fn build_from_w(n: Vec3) -> Self {
        let mut axis = [Vec3::zero(); 3];
        axis[2] = n.unit();
        let a = if axis[2].x().abs() > 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };
        axis[1] = Vec3::cross(axis[2], a).unit();
        axis[0] = Vec3::cross(axis[2], axis[1]);
        Self { axis }
    }
}

impl Index<usize> for Onb {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        if index <= 2 {
            &self.axis[index]
        } else {
            panic!("Unvaild index!");
        }
    }
}

impl IndexMut<usize> for Onb {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index <= 2 {
            &mut self.axis[index]
        } else {
            panic!("Unvaild index!");
        }
    }
}
