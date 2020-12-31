use std::{fmt, ops};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 { x: f64, y: f64, z: f64 }

impl Vec3 {

    pub fn new(_x: f64, _y: f64, _z: f64) -> Vec3 {
        Vec3{ x: _x, y: _y, z: _z}
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x+_rhs.x, self.y+_rhs.y, self.z+_rhs.z)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: f64) -> Vec3 {
        Vec3::new(self.x+_rhs, self.y+_rhs, self.z+_rhs)
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(_rhs.x+self, _rhs.y+self, _rhs.z+self)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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

    }
}
