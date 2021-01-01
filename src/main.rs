
mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use image::{ImageBuffer, RgbImage, Rgb};
use ray::Ray;
use vec3::{Color, Point3, unit_vector};
use hittable::{Hittable, HittableList, HitRecord};
use rand::random;
use sphere::Sphere;

fn imsave(name: &str, width: usize, height: usize, data: Vec<f64>) {

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    let mut i = 0;
    for y in 0..height {
        for x in 0..width {
            let r = (data[i] * 255f64) as u8;
            let g = (data[i+1] * 255f64) as u8;
            let b = (data[i+2] * 255f64) as u8;
            img.put_pixel( x as u32, y as u32, Rgb([r, g, b]));

            i += 3;
        }
    }

    img.save(name).unwrap();

}

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    let mut hit_record = HitRecord::new();
    if world.hit(ray, 0., f64::INFINITY, &mut hit_record) {
        0.5 * (hit_record.normal + Color::new(1., 1., 1.))
    } else {
        let unit_direction = unit_vector(ray.direction);
        let t = 0.5*(unit_direction.y + 1.0);
        (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera
    let camera = Camera::new(16./9.);

    // Render
    let mut data: Vec<f64> = Vec::with_capacity(image_width*image_height*3);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width-1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height-1) as f64;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }
            data.push(pixel_color.x / samples_per_pixel as f64);
            data.push(pixel_color.y / samples_per_pixel as f64);
            data.push(pixel_color.z / samples_per_pixel as f64);
        }
    }

    imsave("render.png", image_width, image_height, data);

}
