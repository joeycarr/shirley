use crate::aabb::{AABB, aabb_compare, surrounding_box};
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use crate::material::Material;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {

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
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hitrec: &mut HitRecord) -> bool;

    fn bounding_box(&self, time0: f64, time1: f64, aabb: &mut AABB) -> bool;
}

pub type HitArc = Arc<dyn Hit + Sync + Send>;

#[derive(Default)]
pub struct HitList {
    pub objects: Vec<HitArc>,
}

impl HitList {
    pub fn add(&mut self, object: HitArc) {
        self.objects.push(object);
    }
}

impl Hit for HitList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hitrec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                hitrec.copy(&temp_rec);
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, time0: f64, time1: f64, aabb: &mut AABB) -> bool {
        let mut temp_box = AABB::default();
        let mut first_box = true;

        for object in &self.objects {
            if object.bounding_box(time0, time1, &mut temp_box) {
                *aabb = if first_box { temp_box } else { surrounding_box(temp_box, *aabb) };
                first_box = false;
            } else {
                return false;
            }
        }

        true
    }
}

pub fn hit_compare(a: HitArc, b: HitArc, axis: usize) -> Ordering {
    let mut box_a = AABB::default();
    let mut box_b = AABB::default();

    // Generating a runtime error seems unfortunate here when we might be able to avoid this using
    // compile time checks
    if (! a.bounding_box(0.0, 0.0, &mut box_a)) || (!b.bounding_box(0.0, 0.0, &mut box_b)) {
        panic!("Attempting to compare an object without a bounding box");
    }

    aabb_compare(box_a, box_b, axis)
}
