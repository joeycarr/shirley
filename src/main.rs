
mod ray;
mod vec3;

use image::{ImageBuffer, RgbImage, Rgb};
use ray::Ray;
use vec3::{Color, Point3, unit_vector, Vec3};

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

fn ray_color(ray: Ray) -> Color {
    let unit_direction = unit_vector(ray.direction);
    let t = 0.5*(unit_direction.y + 1.0);
    return (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
}

fn main() {

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width as f64, 0., 0.);
    let vertical = Vec3::new(0., viewport_height as f64, 0.);
    let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3::new(0., 0., focal_length);

    // Render
    let mut data: Vec<f64> = Vec::with_capacity(image_width*image_height*3);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;

            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(r);
            data.push(pixel_color.x);
            data.push(pixel_color.y);
            data.push(pixel_color.z);
        }
    }

    imsave("render.png", image_width, image_height, data);

}
