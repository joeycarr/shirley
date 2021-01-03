use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};
use crate::materials::Material;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self { center, radius, material }
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> (bool, Option<&Material>) {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0. {
            return (false, Some(&self.material));
        }

        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            return (false, Some(&self.material));
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        (true, Some(&self.material))
    }

}

pub struct SphereList {
    objects: Vec<Sphere>,
}

impl SphereList {
    pub fn new() -> SphereList {
        SphereList{ objects: Vec::new() }
    }

    pub fn add(&mut self, object: Sphere) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> (bool, Option<&Material>) {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        let mut material: Option<&Material> = None;

        for object in &self.objects {
            let tup = object.hit(ray, t_min, closest_so_far, &mut temp_rec);
            let hit: bool = tup.0;
            if hit {
                hit_anything = true;
                material = tup.1;
                closest_so_far = temp_rec.t;
                hit_record.copy(&temp_rec);
            }
        }

        return (hit_anything, material);
    }

}
