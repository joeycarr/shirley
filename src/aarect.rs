use crate::aabb::AABB;
use crate::hit::{Hit, HitArc};
use crate::material::Material;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct XYRect {
    k: f64,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    material: Material,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> HitArc {
        Arc::new(XYRect { x0, x1, y0, y1, k, material })
    }
}

impl Hit for XYRect {
    fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64, hitrec: &mut crate::hit::HitRecord) -> bool {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            false
        } else {
            let x = ray.origin.x + t*ray.direction.x;
            let y = ray.origin.y + t*ray.direction.y;

            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                false
            } else {
                hitrec.t = t;
                hitrec.point = ray.at(t);
                hitrec.u = (x-self.x0) / (self.x1-self.x0);
                hitrec.v = (y-self.y0) / (self.y1-self.y0);
                hitrec.material = Some(Arc::clone(&self.material));
                hitrec.set_face_normal(ray, Vec3::new(0.0, 0.0, 1.0));

                true
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, aabb: &mut AABB) -> bool {
        *aabb = AABB::new(
            Point3::new(self.x0, self.y0, self.k-0.0001),
            Point3::new(self.x1, self.y1, self.k+0.0001),
        );
        true
    }
}

pub struct XZRect {
    k: f64,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    material: Material,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Material) -> HitArc {
        Arc::new(XZRect { x0, x1, z0, z1, k, material })
    }
}

impl Hit for XZRect {
    fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64, hitrec: &mut crate::hit::HitRecord) -> bool {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            false
        } else {
            let x = ray.origin.x + t*ray.direction.x;
            let z = ray.origin.z + t*ray.direction.z;

            if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
                false
            } else {
                hitrec.t = t;
                hitrec.point = ray.at(t);
                hitrec.u = (x-self.x0) / (self.x1-self.x0);
                hitrec.v = (z-self.z0) / (self.z1-self.z0);
                hitrec.material = Some(Arc::clone(&self.material));
                hitrec.set_face_normal(ray, Vec3::new(0.0, 1.0, 0.0));

                true
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, aabb: &mut AABB) -> bool {
        *aabb = AABB::new(
            Point3::new(self.x0, self.k-0.0001, self.z0),
            Point3::new(self.x1, self.k+0.0001, self.z1),
        );
        true
    }
}


pub struct YZRect {
    k: f64,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    material: Material,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Material) -> HitArc {
        Arc::new(YZRect { y0, y1, z0, z1, k, material })
    }
}

impl Hit for YZRect {
    fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64, hitrec: &mut crate::hit::HitRecord) -> bool {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            false
        } else {
            let y = ray.origin.y + t*ray.direction.y;
            let z = ray.origin.z + t*ray.direction.z;

            if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
                false
            } else {
                hitrec.t = t;
                hitrec.point = ray.at(t);
                hitrec.u = (y-self.y0) / (self.y1-self.y0);
                hitrec.v = (z-self.z0) / (self.z1-self.z0);
                hitrec.material = Some(Arc::clone(&self.material));
                hitrec.set_face_normal(ray, Vec3::new(1.0, 0.0, 0.0));

                true
            }
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, aabb: &mut AABB) -> bool {
        *aabb = AABB::new(
            Point3::new(self.k-0.0001, self.y0, self.z0),
            Point3::new(self.k+0.0001, self.y1, self.z1),
        );
        true
    }
}
