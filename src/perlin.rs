use crate::rand::randidx;
use crate::vec3::{dot, Point3, Vec3};

const POINT_COUNT: usize = 256;

pub struct PerlinNoise {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut ranvec = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranvec.push(Vec3::randrange(-1.0, 1.0));
        }
        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();
        PerlinNoise{ ranvec, perm_x, perm_y, perm_z }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(
                        self.perm_x[(i+(di as i32)) as usize & 255] ^
                        self.perm_y[(j+(dj as i32)) as usize & 255] ^
                        self.perm_z[(k+(dk as i32)) as usize & 255]
                    ) as usize]
                }
            }
        }

        trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        // I don't think I like the absolute value here
        accum.abs()
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

fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0f64;

    for iu in 0..2 {
        let i = iu as f64;
        for ju in 0..2 {
            let j = ju as f64;
            for ku in 0..2 {
                let k = ku as f64;
                let weight_v = Vec3::new(u-i, v-j, w-k);
                accum +=
                    (i*u + (1.0-i)*(1.0-u))*
                    (j*v + (1.0-j)*(1.0-v))*
                    (k*w + (1.0-k)*(1.0-w))*
                    dot(c[iu][ju][ku], weight_v);
            }
        }
    }

    accum
}
