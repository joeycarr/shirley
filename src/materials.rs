use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, dot, unit_vector, Vec3};
use rand;


pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.*dot(v, n)*n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64 ) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = n * -(1f64 - r_out_perp.length_squared()).abs().sqrt();
    return r_out_perp + r_out_parallel;
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0-ref_idx) / (1.0+ref_idx);
    let r0 = r0*r0;
    return r0 + (1.0-r0)*(1.0 -cosine).powi(5);
}


pub trait Material {
    fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray) -> bool;
}


pub struct Lambertian{
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian{ albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: Ray, hit_record: &mut HitRecord, attenuation: &mut Color, ray_scattered: &mut Ray) -> bool {
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

pub struct Metal{
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal{ albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit_record: &mut HitRecord, attenuation: &mut Color, ray_scattered: &mut Ray) -> bool {
        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        let reflected = reflect(unit_vector(ray_in.direction), hit_record.normal);
        ray_scattered.origin.copy(hit_record.point);
        ray_scattered.direction.copy(reflected + fuzz*Vec3::random_in_unit_sphere());
        attenuation.copy(self.albedo);
        return true;
    }
}

pub struct Dielectric{
    ior: f64
}

impl Dielectric {
    pub fn new(ior: f64) -> Dielectric {
        Dielectric{ ior }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit_record: &mut HitRecord, attenuation: &mut Color, ray_scattered: &mut Ray) -> bool {
        attenuation.set(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face { 1.0 / self.ior } else { self.ior };

        let unit_direction = unit_vector(ray_in.direction);
        let cos_theta = dot(-unit_direction, hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random::<f64>() {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        ray_scattered.origin.copy(hit_record.point);
        ray_scattered.direction.copy(direction);
        return true;
    }
}
