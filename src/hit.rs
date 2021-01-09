use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use crate::material::Material;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Material>,
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
        match other.material {
            Some(ref material) => {
                self.material = Some(Arc::clone(material));
            }
            None => {
                self.material = None;
            }
        }
    }
}

pub trait Hit {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub type HitArc = Arc<dyn Hit + Sync + Send>;

pub struct HitList {
    objects: Vec<Arc<dyn Hit + Sync + Send>>,
}

impl HitList {
    pub fn new() -> HitList {
        HitList{ objects: Vec::new() }
    }

    pub fn add(&mut self, object: HitArc) {
        self.objects.push(object);
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
