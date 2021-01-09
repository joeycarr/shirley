use crate::hit::{Hit, HitArc, HitRecord};
use crate::ray::Ray;
use crate::vec3::{dot, Point3};
use crate::material::Material;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> HitArc {
        Arc::new(Sphere{ center, radius, material })
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hitrec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            return false;
        }

        hitrec.t = root;
        hitrec.point = ray.at(hitrec.t);
        let outward_normal = (hitrec.point - self.center) / self.radius;
        hitrec.set_face_normal(ray, outward_normal);

        hitrec.material = Some(self.material.clone());

        true
    }

}
