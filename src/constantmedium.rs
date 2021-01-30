use crate::aabb::AABB;
use crate::hit::{Hit, HitArc, HitRecord};
use crate::material::{Material, Isotropic};
use crate::rand::rf64;
use crate::ray::Ray;
use crate::vec3::{Color,Vec3};

use std::sync::Arc;

pub struct ConstantMedium {
    boundary: HitArc,
    phase_function: Material,
    neg_inv_density: f64,
}

impl ConstantMedium {

    pub fn new(boundary: HitArc, density: f64, albedo: Color) -> HitArc {
        Arc::new(ConstantMedium {
            boundary,
            neg_inv_density: -1.0/density,
            phase_function: Isotropic::from_color(albedo),
        })
    }

}

impl Hit for ConstantMedium {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hitrec: &mut HitRecord) -> bool {
        const DEBUG: bool = false;
        let debug = DEBUG && rf64() < 0.00001;

        let mut rec1: HitRecord = Default::default();
        let mut rec2: HitRecord = Default::default();

        if self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY, &mut rec1) {
            if debug {
                println!("rec1.t == {:?}", rec1.t);
                println!("ray.at(rec1.t) == {:?}", ray.at(rec1.t));
            }

            if self.boundary.hit(ray, rec1.t+0.0001, f64::INFINITY, &mut rec2) {

                rec1.t = if rec1.t < t_min { t_min } else { rec1.t };
                rec2.t = if rec2.t > t_max { rec2.t } else { rec2.t };

                if rec1.t < rec2.t {
                    rec1.t = if rec1.t < 0.0 { 0.0 } else { rec1.t };

                    let ray_length = ray.direction.length();
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                    let hit_distance = self.neg_inv_density * rf64().ln();

                    if hit_distance < distance_inside_boundary {

                        hitrec.t = rec1.t + hit_distance / ray_length;
                        hitrec.point = ray.at(hitrec.t);

                        hitrec.normal = Vec3::new(1.0, 0.0, 0.0);
                        hitrec.front_face = true;
                        hitrec.material = Some(Arc::clone(&self.phase_function));

                        if debug { println!("All the way down"); }
                        true

                    } else {
                        if debug { println!("Not inside boundary."); }
                        false
                    }
                } else {
                    if debug { println!("rec1.t >= rec2.t"); }
                    false
                }
            } else {
                if debug { println!("no hit inner"); }
                false
            }
        } else {
            //if debug { println!("no hit outer"); }
            false
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64, aabb: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, aabb)
    }
}
