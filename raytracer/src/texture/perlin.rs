use crate::{
    rtweekend::{random_double_unit, random_int},
    Point3,
};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ranfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranfloat = [0.; POINT_COUNT];
        for i in 0..POINT_COUNT {
            ranfloat[i] = random_double_unit();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
        for it in 1..n {
            let i = n - it;
            let target = random_int(0, i as isize) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (((4. * p.x()) as isize) & 255_isize) as usize;
        let j = (((4. * p.y()) as isize) & 255_isize) as usize;
        let k = (((4. * p.z()) as isize) & 255_isize) as usize;
        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}
