use crate::hit::{Hit, HitArc, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};
use std::sync::Arc;

pub struct MovingSphere {
    center0: Point3, // center at time0
    center1: Point3, // center at time1
    time0: f64,
    time1: f64,
    radius: f64,
    material: Material,
}

impl MovingSphere {
    pub fn new(
        center0: Point3, center1: Point3,
        time0: f64, time1: f64,
        radius: f64,
        material: Material
    ) -> HitArc {
        Arc::new(MovingSphere { center0, center1, time0, time1, radius, material })
    }

    fn center(&self, time: f64) -> Point3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)*(self.center1 - self.center0))
    }
}

impl Hit for MovingSphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hitrec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center(ray.time);
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
        let outward_normal = (hitrec.point - self.center(ray.time)) / self.radius;
        hitrec.set_face_normal(ray, outward_normal);

        // Note that we're cloning the enclousing Arc, not the material itself.
        hitrec.material = Some(self.material.clone());

        true
    }
}
