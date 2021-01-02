use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, dot, unit_vector, Vec3};


pub trait Material {
    fn scatter(
        self: &Self,
        ray_in: Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray) -> bool;
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.*dot(v, n)*n
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian{ albedo }
    }
}

impl Material for Lambertian {

    fn scatter(
        self: &Self,
        ray_in: Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray) -> bool {
            let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

            if scatter_direction.near_zero() {
                scatter_direction = hit_record.normal;
            }

            ray_scattered.origin.copy(hit_record.point);
            ray_scattered.direction.copy(scatter_direction);
            attenuation.copy(self.albedo);
            return true;
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal{ albedo }
    }
}

impl Material for Metal {
    fn scatter(
        self: &Self,
        ray_in: Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray) -> bool {
            let reflected = reflect(unit_vector(ray_in.direction), hit_record.normal);
            ray_scattered.origin.copy(hit_record.point);
            ray_scattered.direction.copy(reflected);
            attenuation.copy(self.albedo);
            return true;
    }
}
