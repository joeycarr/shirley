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
        self.u = other.u;
        self.v = other.v;
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

pub struct Translate {
    thing: HitArc,
    offset: Vec3,
}

impl Translate {
    pub fn new(thing: HitArc, offset: Vec3) -> HitArc {
        Arc::new(Translate{ thing, offset })
    }
}

impl Hit for Translate {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hitrec: &mut HitRecord) -> bool {
        let moved = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        if self.thing.hit(moved, t_min, t_max, hitrec) {
            hitrec.point += self.offset;
            hitrec.set_face_normal(moved, hitrec.normal);
            true
        } else {
            false
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64, aabb: &mut AABB) -> bool {
        if self.thing.bounding_box(time0, time1, aabb) {
            aabb.min += self.offset;
            aabb.max += self.offset;
            true
        } else {
            false
        }
    }
}

pub struct RotateY {
    thing: HitArc,
    sin_theta: f64,
    cos_theta: f64,
    aabb: Option<AABB>
}

impl RotateY {
    pub fn new(thing: HitArc, theta: f64) -> HitArc {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let mut bbox = AABB::default();
        let aabb;
        if thing.bounding_box(0.0, 1.0, &mut bbox) {
            let mut min = [f64::INFINITY, f64::INFINITY, f64::INFINITY];
            let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

            for i in 0..2 {
                let i = i as f64;
                for j in 0..2 {
                    let j = j as f64;
                    for k in 0..2 {
                        let k = k as f64;
                        let x = i*bbox.max.x + (1.0-i)*bbox.min.x;
                        let y = j*bbox.max.y + (1.0-j)*bbox.min.y;
                        let z = k*bbox.max.z + (1.0-k)*bbox.min.z;

                        let newx =  cos_theta*x + sin_theta*z;
                        let newz = -sin_theta*x + cos_theta*z;

                        let tester = [newx, y, newz];

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
            aabb = Some(AABB::new(
                Point3::from_array(min),
                Point3::from_array(max),
            ));
        } else {
            aabb = None
        }
        Arc::new(RotateY{ thing, sin_theta, cos_theta, aabb })
    }
}

impl Hit for RotateY {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hitrec: &mut HitRecord) -> bool {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin.x = self.cos_theta*ray.origin.x - self.sin_theta*ray.origin.z;
        origin.z = self.sin_theta*ray.origin.x + self.cos_theta*ray.origin.z;

        direction.x = self.cos_theta*ray.direction.x - self.sin_theta*ray.direction.z;
        direction.z = self.sin_theta*ray.direction.x + self.cos_theta*ray.direction.z;

        let rotated = Ray::new(origin, direction, ray.time);

        if self.thing.hit(rotated, t_min, t_max, hitrec) {
            let mut point = hitrec.point;
            let mut normal = hitrec.normal;

            point.x =  self.cos_theta*hitrec.point.x + self.sin_theta*hitrec.point.z;
            point.z = -self.sin_theta*hitrec.point.x + self.cos_theta*hitrec.point.z;

            normal.x = self.cos_theta*hitrec.normal.x + self.sin_theta*hitrec.normal.z;
            normal.z = self.sin_theta*hitrec.normal.x + self.cos_theta*hitrec.normal.z;

            hitrec.point = point;
            hitrec.set_face_normal(rotated, normal);

            true
        } else {
            false
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, aabb: &mut AABB) -> bool {
        match self.aabb {
            Some(bbox) => {
                *aabb = bbox;
                true
            },
            None => {
                false
            },
        }
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
