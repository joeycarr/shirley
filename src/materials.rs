use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, dot, unit_vector, Vec3};

pub enum Material {
    Lambertian{ albedo: Color },
    Metal{ albedo: Color },
}

impl Material {

    pub fn new_lambertian(r: f64, g: f64, b: f64) -> Material {
        Material::Lambertian{ albedo: Color::new(r, g, b) }
    }

    pub fn new_metal(r: f64, g: f64, b: f64) -> Material {
        Material::Metal{ albedo: Color::new(r, g, b) }
    }

    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray) -> bool {

            match self {
                Material::Lambertian{ albedo } => scatter_lambertian(albedo, ray_in, hit_record, attenuation, ray_scattered),
                Material::Metal{ albedo } => scatter_metal(albedo, ray_in, hit_record, attenuation, ray_scattered)
            }
        }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.*dot(v, n)*n
}

fn scatter_lambertian(
    albedo: &Color,
    _ray_in: Ray,
    hit_record: &mut HitRecord,
    attenuation: &mut Color,
    ray_scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        ray_scattered.origin.copy(hit_record.point);
        ray_scattered.direction.copy(scatter_direction);
        attenuation.copy(*albedo); // creating a dumb copy, I think?
        return true;
}

fn scatter_metal(
    albedo: &Color,
    ray_in: Ray,
    hit_record: &mut HitRecord,
    attenuation: &mut Color,
    ray_scattered: &mut Ray) -> bool {
        let reflected = reflect(unit_vector(ray_in.direction), hit_record.normal);
        ray_scattered.origin.copy(hit_record.point);
        ray_scattered.direction.copy(reflected);
        attenuation.copy(*albedo);
        return true;
}
