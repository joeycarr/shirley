use crate::perlin::PerlinNoise;
use crate::vec3::{Color, Point3};
use std::sync::Arc;

pub type Texture = Arc<dyn Value + Sync + Send>;

pub trait Value {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

#[derive(Default)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Texture {
        Arc::new(SolidColor{ color })
    }
}

impl Value for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color
    }
}

pub struct Checker {
    even: Texture,
    odd: Texture,
}

impl Checker {
    pub fn new(odd: Color, even: Color) -> Texture {
        Arc::new(Checker{
            odd: SolidColor::new(odd),
            even: SolidColor::new(even),
        })
    }
}

impl Value for Checker {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0*p.x).sin() * (10.0*p.y).sin() * (10.0*p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct Perlin {
    noise: PerlinNoise,
    scale: f64,
}

impl Perlin {
    pub fn new(scale: f64) -> Texture {
        Arc::new(Perlin{ noise: PerlinNoise::new(), scale })
    }
}

impl Value for Perlin {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0*self.noise.turb(p, 7)).sin())
    }
}
