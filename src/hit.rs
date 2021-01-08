use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use crate::materials::Material;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material + Sync + Send>>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            point: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            front_face: false,
            material: None
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }

    pub fn copy(&mut self, other: &HitRecord) {
        self.point = other.point;
        self.normal = other.normal;
        self.t = other.t;
        self.front_face = other.front_face;
    }
}

pub trait Hit {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct HitList {
    objects: Vec<Arc<Box<dyn Hit + Sync + Send>>>,
}

impl HitList {
    pub fn new() -> HitList {
        HitList{ objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hit + Sync + Send>) {
        self.objects.push(Arc::new(object));
    }
}

impl Hit for HitList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                hit_record.copy(&temp_rec);
            }
        }

        return hit_anything;
    }
}
