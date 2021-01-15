use crate::perlin::PerlinNoise;
use crate::vec3::{Color, Point3};
use image::io::Reader as ImageReader;
use image::RgbImage;
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

pub struct Image {
    data: RgbImage,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(filename: &str) -> Texture {
        // Why is this not working with the question mark operator?
        let data = ImageReader::open(filename).unwrap().decode().unwrap().into_rgb8();
        let width = data.width();
        let height = data.height();
        Arc::new(Image{ data, width, height })
    }
}

impl Value for Image {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let u = u.max(0.0).min(1.0);
        let v = 1.0 - v.max(0.0).min(1.0);

        let i = (u * self.width as f64) as u32;
        let j = (v * self.height as f64) as u32;

        let i = i.min(self.width-1);
        let j = j.min(self.height-1);

        const COLOR_SCALE: f64 = 1.0 / 255.0;

        let p = self.data.get_pixel(i, j);

        Color::new(
            COLOR_SCALE * p[0] as f64,
            COLOR_SCALE * p[1] as f64,
            COLOR_SCALE * p[2] as f64,
        )
    }
}
