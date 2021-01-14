mod aabb;
mod bvh;
mod camera;
mod hit;
mod material;
mod movingsphere;
mod rand;
mod ray;
mod sphere;
mod texture;
mod vec3;

use bvh::BVHNode;
use camera::Camera;
use hit::{Hit, HitList, HitRecord};
use image::{ImageBuffer, RgbImage, Rgb};
use ray::Ray;
use crate::rand::rf64;
use sphere::Sphere;
use movingsphere::MovingSphere;
use material::{Dielectric, Lambertian, Metal};
use crate::rand::randrange;
use std::thread;
use std::sync::Arc;
use texture::Checker;
use vec3::{Color, Point3, unit_vector, Vec3};

/**
 * Averages the corresponding indices in the given list of vectors. The result is a vector as long
 * as the shortest inner vector in the data. This is destructive. The input vector is collapsed
 * down to a single inner vector that's the sum of its contents.
 */
fn average(mut data: Vec<Vec<f64>>) -> Vec<f64> {
    let denomenator = data.len();
    while data.len() > 1 {
        // I'm confident this unwrap won't panic (famous last words?)
        let vec1 = data.pop().unwrap();
        let vec2 = data.pop().unwrap();
        let it = vec1.iter().zip(vec2.iter());

        let mut sum: Vec<f64> = Vec::with_capacity(vec1.len().min(vec2.len()));
        for (a, b) in it {
            sum.push(a + b);
        }

        data.push(sum);
    }

    let summed = data.pop().unwrap();
    let averaged = summed.iter().map(|x| x/denomenator as f64).collect::<Vec<f64>>();

    averaged
}

fn imsave(name: &str, width: usize, height: usize, data: Vec<f64>) {

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    let mut i = 0;
    for y in 0..height {
        for x in 0..width {
            let r = (data[i] * 256.) as u8;
            let g = (data[i+1] * 256.) as u8;
            let b = (data[i+2] * 256.) as u8;
            img.put_pixel( x as u32, y as u32, Rgb([r, g, b]));

            i += 3;
        }
    }

    img.save(name).unwrap();

}

fn ray_color(ray: Ray, world: &HitList, depth: usize) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    let mut hitrec = HitRecord::default();
    if world.hit(ray, 0.001, f64::INFINITY, &mut hitrec) {
        let mut ray_scattered = Ray::new(
            Point3::new(0., 0., 0.),
            Vec3::new(0., 0., 0.),
            0.0
        );
        let mut attenuation = Color::new(0., 0., 0.);

        if let Some(ref material) = hitrec.material {
            let material = Arc::clone(material);
            material.scatter(ray, &mut hitrec, &mut attenuation, &mut ray_scattered);
            return attenuation * ray_color(ray_scattered, world, depth-1);
        }

        let target = hitrec.point + Vec3::random_in_hemisphere(hitrec.normal);
        0.5 * ray_color(Ray::new(hitrec.point, target - hitrec.point, ray.time), world, depth-1)
    } else {
        let unit_direction = unit_vector(ray.direction);
        let t = 0.5*(unit_direction.y + 1.0);
        (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
    }
}

fn random_scene() -> HitList {
    let mut world = HitList::default();

    let ground_material = Lambertian::new(Checker::new(
        Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)
    ));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rf64();
            let center = Point3::new(
                a as f64 + 0.9*rf64(),
                0.2,
                b as f64 + 0.9*rf64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                match choose_mat {
                    x if x < 0.8 => {
                        let material = Lambertian::from_color(Color::new(
                            rf64()*rf64(),
                            rf64()*rf64(),
                            rf64()*rf64(),
                        ));
                        let center2 = center + Vec3::new(0.0, randrange(0.0, 0.5), 0.0);
                        world.add(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, material));
                    }
                    x if x < 0.95 => {
                        let material = Metal::new(Color::new(
                                rf64()*rf64(),
                                rf64()*rf64(),
                                rf64()*rf64(),
                            ),
                            randrange(0.0, 0.5)
                        );
                        world.add(Sphere::new(center, 0.2, material));
                    }
                    _ => {
                        let material = Dielectric::new(1.5);
                        world.add(Sphere::new(center, 0.2, material));
                    }
                };
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::from_color(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let bvh = BVHNode::from_hitlist(&world, 0.0, 1.0);
    let mut world = HitList::default();
    world.add(Arc::new(bvh));
    world
}

fn two_spheres() -> HitList {
    let mut objects = HitList::default();

    let checker = Checker::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    objects.add(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, Lambertian::new(Arc::clone(&checker))));
    objects.add(Sphere::new(Point3::new(0.0,  10.0, 0.0), 10.0, Lambertian::new(Arc::clone(&checker))));

    objects
}

fn render(
    world: &HitList,
    camera: &Camera,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize
) -> Vec<f64> {
    let mut data: Vec<f64> = Vec::with_capacity(image_width*image_height*3);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rf64()) / (image_width-1) as f64;
                let v = (j as f64 + rf64()) / (image_height-1) as f64;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            let scale = 1. / samples_per_pixel as f64;
            data.push((pixel_color.x * scale).sqrt());
            data.push((pixel_color.y * scale).sqrt());
            data.push((pixel_color.z * scale).sqrt());
        }
    }

    data
}

fn main() {

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let lookfrom: Point3;
    let lookat: Point3;
    let vfov: f64;
    let aperture: f64;

    let world: HitList = match 2 {
        1 => {
            lookfrom = Point3::new(13.0 ,2.0 ,3.0);
            lookat = Point3::new(0.0 ,0.0 ,0.0);
            vfov = 20.0;
            aperture = 0.1;
            random_scene()
        }
        // make 2 the catchall for now so match is exhaustive
        _ => {
            lookfrom = Point3::new(13.0 ,2.0 ,3.0);
            lookat = Point3::new(0.0 ,0.0 ,0.0);
            vfov = 20.0;
            aperture = 0.0;
            two_spheres()
        }
    };

    // Camera
    let vup = Vec3::new(0.0, 1.0,  0.0);
    let dist_to_focus = 10.0;

    let camera = Camera::new(
        lookfrom, lookat, vup,
        vfov, // vertical fov
        aspect_ratio, aperture, dist_to_focus,
        0.0, 1.0 // shutter time
    );

    // Render

    let worldref = Arc::new(world);
    let camref = Arc::new(camera);

    let thread_count = 8;
    let mut results: Vec<Vec<f64>> = Vec::new();
    let mut threads: Vec<thread::JoinHandle<Vec<f64>>> = Vec::new();

    let samples_per_thread = (samples_per_pixel as f64 / thread_count as f64).ceil() as usize;
    println!("Running {} threads at {} samples for a total of {} samples per pixel",
        thread_count,
        samples_per_thread,
        samples_per_thread * thread_count
    );

    for i in 0..thread_count {
        // These will be moved into the thread...
        let worldref = Arc::clone(&worldref);
        let camref = Arc::clone(&camref);

        let handle = thread::spawn(move || {
            println!("Starting thread #{} of {}", i, thread_count);
            let data = render(&worldref, &camref, image_width, image_height, samples_per_thread, max_depth);
            println!("Completed thread #{} of {}", i, thread_count);
            data
        });

        threads.push(handle);
    }

    for handle in threads {
        match handle.join() {
            Ok(result) => {
                results.push(result);
            }
            Err(error) => {
                println!("Thread encountered an error: {:?}", error);
            }
        }
    }

    let image = average(results);

    imsave("render.png", image_width, image_height, image);

}
