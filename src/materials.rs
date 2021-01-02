use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, dot, unit_vector, Vec3};

pub enum Material {
    Lambertian{ albedo: Color },
    Metal{ albedo: Color, fuzz: f64 },
    Dielectric{ ior: f64 },
}

impl Material {

    pub fn new_dielectric(ior: f64) -> Material {
        Material::Dielectric{ ior }
    }

    pub fn new_lambertian(r: f64, g: f64, b: f64) -> Material {
        Material::Lambertian{ albedo: Color::new(r, g, b) }
    }

    pub fn new_metal(rgb: (f64, f64, f64), fuzz: f64) -> Material {
        let (r, g, b) = rgb;
        Material::Metal{ albedo: Color::new(r, g, b), fuzz }
    }

    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &mut HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray) -> bool {

        match self {
            Material::Lambertian{ albedo } =>
                scatter_lambertian(*albedo, ray_in, hit_record, attenuation, ray_scattered),
            Material::Metal{ albedo, fuzz } =>
                scatter_metal(*albedo, *fuzz, ray_in, hit_record, attenuation, ray_scattered),
            Material::Dielectric{ ior } =>
                scatter_dielectric(*ior, ray_in, hit_record, attenuation, ray_scattered),
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.*dot(v, n)*n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64 ) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = n * -(1f64 - r_out_perp.length_squared()).abs().sqrt();
    return r_out_perp + r_out_parallel;
}

fn scatter_dielectric(
    ior: f64,
    ray_in: Ray,
    hit_record: &mut HitRecord,
    attenuation: &mut Color,
    ray_scattered: &mut Ray) -> bool {
        attenuation.set(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face { 1.0 / ior } else { ior };

        let unit_direction = unit_vector(ray_in.direction);
        let refracted = refract(unit_direction, hit_record.normal, refraction_ratio);

        ray_scattered.origin.copy(hit_record.point);
        ray_scattered.direction.copy(refracted);
        return true;
}

fn scatter_lambertian(
    albedo: Color,
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
        attenuation.copy(albedo);
        return true;
}

fn scatter_metal(
    albedo: Color,
    fuzz: f64,
    ray_in: Ray,
    hit_record: &mut HitRecord,
    attenuation: &mut Color,
    ray_scattered: &mut Ray) -> bool {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        let reflected = reflect(unit_vector(ray_in.direction), hit_record.normal);
        ray_scattered.origin.copy(hit_record.point);
        ray_scattered.direction.copy(reflected + fuzz*Vec3::random_in_unit_sphere());
        attenuation.copy(albedo);
        return true;
}
