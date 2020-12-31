use std::{fmt, ops};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{x, y, z}
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x*v.x + u.y*v.y + u.z*v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    return Vec3::new(
        u.y*v.z - u.z*v.y,
        u.z*v.x - u.x*v.z,
        u.x*v.y - u.y*v.x
    );
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x+rhs.x, self.y+rhs.y, self.z+rhs.z)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x+rhs, self.y+rhs, self.z+rhs)
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x+self, rhs.y+self, rhs.z+self)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x*rhs.x, self.y*rhs.y, self.z*rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x*self, rhs.y*self, rhs.z*self)
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x/rhs, self.y/rhs, self.z/rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x-rhs.x, self.y-rhs.y, self.z-rhs.z)
    }
}


#[cfg(test)]
mod tests {

    use crate::vec3::Vec3;

    #[test]
    fn length_squared() {
        let v = Vec3::new(1., 1., 1.);
        assert_eq!(v.length_squared(), 3f64)
    }

    #[test]
    fn length() {
        let v = Vec3::new(1., 1., 1.);
        assert_eq!(v.length(), 3f64.sqrt())
    }

    #[test]
    fn addition() {
        let v1 = Vec3::new(1., 1., 1.);
        let v2 = Vec3::new(2., 3., 4.);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 3.);
        assert_eq!(v3.y, 4.);
        assert_eq!(v3.z, 5.);

        let v4 = v3 + 5.;
        assert_eq!(v4.x, 8.);
        assert_eq!(v4.y, 9.);
        assert_eq!(v4.z, 10.);

        let mut v5 = 5. + v4;
        assert_eq!(v5.x, 13.);
        assert_eq!(v5.y, 14.);
        assert_eq!(v5.z, 15.);

        v5 += v5;
        assert_eq!(v5.x, 26.);
        assert_eq!(v5.y, 28.);
        assert_eq!(v5.z, 30.);

        v5 += 10.;
        assert_eq!(v5.x, 36.);
        assert_eq!(v5.y, 38.);
        assert_eq!(v5.z, 40.);
    }

    #[test]
    fn multiplication() {
        let v1 = Vec3::new(2., 2., 2.);
        let v2 = Vec3::new(2., 3., 4.);
        let v3 = v1 * v2;
        assert_eq!(v3.x, 4.);
        assert_eq!(v3.y, 6.);
        assert_eq!(v3.z, 8.);

        let v4 = v3 * 2.;
        assert_eq!(v4.x, 8.);
        assert_eq!(v4.y, 12.);
        assert_eq!(v4.z, 16.);

        let mut v5 = 2. * v4;
        assert_eq!(v5.x, 16.);
        assert_eq!(v5.y, 24.);
        assert_eq!(v5.z, 32.);

        v5 *= v5;
        assert_eq!(v5.x, 256f64);
        assert_eq!(v5.y, 576f64);
        assert_eq!(v5.z, 1024f64);

        v5 *= 10.;
        assert_eq!(v5.x, 2560f64);
        assert_eq!(v5.y, 5760f64);
        assert_eq!(v5.z, 10240f64);
    }
}
