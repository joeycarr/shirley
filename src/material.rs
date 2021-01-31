use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Color, dot, Point3, unit_vector, Vec3};
use rand;
use std::sync::Arc;


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
    return r0 + (1.0-r0)*(1.0 - cosine).powi(5);
}

pub trait Scatter {
    fn scatter(
        &self,
        ray_in: Ray,
        hitrec: &mut HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::default()
    }

}

pub type Material = Arc<dyn Scatter + Sync + Send>;

pub struct Lambertian{
    albedo: Texture,
}

impl Lambertian {
    pub fn from_color(albedo: Color) -> Material {
        Arc::new(Lambertian{ albedo: SolidColor::new(albedo) })
    }

    pub fn new(albedo: Texture) -> Material {
        Arc::new(Lambertian{ albedo })
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in: Ray, hitrec: &mut HitRecord, attenuation: &mut Color, ray_scattered: &mut Ray) -> bool {
        let mut scatter_direction = hitrec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hitrec.normal;
        }

        ray_scattered.origin.copy(hitrec.point);
        ray_scattered.direction.copy(scatter_direction);
        ray_scattered.time = ray_in.time;
        attenuation.copy(self.albedo.value(hitrec.u, hitrec.v, hitrec.point));
        return true;
    }
}

pub struct Metal{
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Material {
        Arc::new(Metal{ albedo, fuzz })
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: Ray, hitrec: &mut HitRecord, attenuation: &mut Color, ray_scattered: &mut Ray) -> bool {
        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        let reflected = reflect(unit_vector(ray_in.direction), hitrec.normal);
        ray_scattered.origin.copy(hitrec.point);
        ray_scattered.direction.copy(reflected + fuzz*Vec3::random_in_unit_sphere());
        ray_scattered.time = ray_in.time;
        attenuation.copy(self.albedo);
        return true;
    }
}

pub struct Dielectric{
    ior: f64
}

impl Dielectric {
    pub fn new(ior: f64) -> Material {
        Arc::new(Dielectric{ ior })
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: Ray, hitrec: &mut HitRecord, attenuation: &mut Color, ray_scattered: &mut Ray) -> bool {
        attenuation.set(1.0, 1.0, 1.0);
        let refraction_ratio = if hitrec.front_face { 1.0 / self.ior } else { self.ior };

        let unit_direction = unit_vector(ray_in.direction);
        let cos_theta = dot(-unit_direction, hitrec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random::<f64>() {
            reflect(unit_direction, hitrec.normal)
        } else {
            refract(unit_direction, hitrec.normal, refraction_ratio)
        };

        ray_scattered.origin.copy(hitrec.point);
        ray_scattered.direction.copy(direction);
        ray_scattered.time = ray_in.time;
        return true;
    }
}

pub struct DiffuseLight {
    emit: Texture,
}

impl DiffuseLight {
    pub fn new(emit: Texture) -> Material {
        Arc::new(DiffuseLight{ emit })
    }
}

impl Scatter for DiffuseLight {
    fn scatter(
        &self,
        _ray_in: Ray,
        _hitrec: &mut HitRecord,
        _attenuation: &mut Color,
        _ray_scattered: &mut Ray) -> bool {

        false
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Texture,
}

impl Isotropic {
    pub fn new(albedo: Texture) -> Material {
        Arc::new(Isotropic{ albedo })
    }

    pub fn from_color(color: Color) -> Material {
        Isotropic::new(SolidColor::new(color))
    }
}

impl Scatter for Isotropic {
    fn scatter(
        &self,
        ray_in: Ray,
        hitrec: &mut HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray) -> bool {

        *ray_scattered = Ray::new(hitrec.point, Vec3::random_in_unit_sphere(), ray_in.time);
        *attenuation = self.albedo.value(hitrec.u, hitrec.v, hitrec.point);

        true
    }
}
