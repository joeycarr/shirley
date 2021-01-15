use crate::rand::{rf64, randidx};
use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct PerlinNoise {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut ranfloat = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranfloat.push(rf64());
        }
        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();
        PerlinNoise{ ranfloat, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = (((4.0f64*p.x) as i32) & 255) as usize;
        let j = (((4.0f64*p.y) as i32) & 255) as usize;
        let k = (((4.0f64*p.z) as i32) & 255) as usize;

        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p: Vec<usize> = Vec::with_capacity(POINT_COUNT);

    for i in 0..POINT_COUNT {
        p.push(i);
    }

    permute(&mut p);

    p
}

fn permute(p: &mut Vec<usize>) {
    let n = p.len() - 1;
    for i in (1..n).rev() {
        let target = randidx(0, i);
        p.swap(i, target);
    }
}
