mod aabb;
mod aarect;
mod box3d;
mod bvh;
mod camera;
mod constantmedium;
mod hit;
mod material;
mod movingsphere;
mod perlin;
mod rand;
mod ray;
mod sphere;
mod texture;
mod vec3;

use aarect::{XYRect, XZRect, YZRect};
use box3d::Box3D;
use bvh::BVHNode;
use camera::Camera;
use constantmedium::ConstantMedium;
use hit::{Hit, HitList, HitRecord, Translate, RotateY};
use image::{ImageBuffer, RgbImage, Rgb};
use ray::Ray;
use crate::rand::rf64;
use sphere::Sphere;
use movingsphere::MovingSphere;
use material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::rand::randrange;
use std::thread;
use std::sync::Arc;
use texture::{Checker, Perlin, Image, SolidColor};
use vec3::{Color, Point3, Vec3};

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

fn ray_color(ray: Ray, world: &HitList, depth: usize, background: Color) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    let mut hitrec = HitRecord::default();
    if world.hit(ray, 0.001, f64::INFINITY, &mut hitrec) {
        let mut ray_scattered = Ray::default();
        let mut attenuation = Color::new(0., 0., 0.);

        if let Some(ref material) = hitrec.material {
            let material = Arc::clone(&material);
            let emitted = material.emitted(hitrec.u, hitrec.v, hitrec.point);

            if material.scatter(ray, &mut hitrec, &mut attenuation, &mut ray_scattered) {
                return emitted + attenuation * ray_color(ray_scattered, world, depth-1, background);
            } else {
                return emitted;
            }
        } else {
            panic!("If something in the world hits, then it must update the material on the hit record.");
        }

    } else {
        return background;
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

fn two_perlin_spheres() -> HitList {
    let mut objects = HitList::default();

    let pertext = Perlin::new(4.0);
    objects.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(Arc::clone(&pertext))));
    objects.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(Arc::clone(&pertext))));

    objects
}

fn earth() -> HitList {
    let earth_texture = Image::new("textures/earthmap.jpg");
    let earth_material = Lambertian::new(earth_texture);
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_material);

    let mut objects = HitList::default();
    objects.add(globe);

    objects
}

fn simple_light() -> HitList {
    let mut objects = HitList::default();

    let pertext = Perlin::new(4.0);
    objects.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new(Arc::clone(&pertext))));
    objects.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(Arc::clone(&pertext))));

    let difflight = DiffuseLight::new(SolidColor::new(Color::new(4.0, 4.0, 4.0)));
    objects.add(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight));

    objects
}

fn cornell_box() -> HitList {
    let mut objects = HitList::default();

    let red = Lambertian::new(SolidColor::from_rgb(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::from_rgb(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::from_rgb(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::from_rgb(15.0, 15.0, 15.0));

    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));

    objects.add(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light));
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&white)));
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white)));

    objects.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white)));

    let box1 = Box3D::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), Arc::clone(&white));
    let box1 = RotateY::new(box1, 15f64.to_radians());
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    objects.add(box1);

    let box2 = Box3D::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), Arc::clone(&white));
    let box2 = RotateY::new(box2, -18f64.to_radians());
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));
    objects.add(box2);

    objects
}

fn cornell_smoke() -> HitList {
    let mut objects = HitList::default();

    let red = Lambertian::new(SolidColor::from_rgb(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::from_rgb(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::from_rgb(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::from_rgb(7.0, 7.0, 7.0));

    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));

    objects.add(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light));
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&white)));
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white)));

    objects.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white)));

    let box1 = Box3D::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), Arc::clone(&white));
    let box1 = RotateY::new(box1, 15f64.to_radians());
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));

    let box2 = Box3D::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), Arc::clone(&white));
    let box2 = RotateY::new(box2, -18f64.to_radians());
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));

    objects.add(ConstantMedium::new(box1, 0.01, Color::new(0.0, 0.0, 0.0)));
    objects.add(ConstantMedium::new(box2, 0.01, Color::new(1.0, 1.0, 1.0)));

    objects
}

fn sphere_volume_test() -> HitList {
    let mut objects = HitList::default();

    objects.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::from_color(Color::new(0.73, 0.73, 0.73))));

    let boundary = Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Dielectric::new(1.5));
    objects.add(Arc::clone(&boundary));
    let cm = ConstantMedium::new(Arc::clone(&boundary), 0.2, Color::new(0.2, 0.4, 0.9));
    objects.add(cm);

    objects.add(Sphere::new(Point3::new(2.0, 0.5, 2.0), 0.5, DiffuseLight::new(SolidColor::from_rgb(10.0, 10.0, 10.0))));

    objects
}

fn final_scene() -> HitList {
    let mut boxes1: HitList = Default::default();

    let ground = Lambertian::from_color(Color::new(0.48, 0.83, 0.53));

    const BOXES_PER_SIDE: usize = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0f64;
            let x0: f64 = -1000.0 + (i as f64)*w;
            let z0: f64 = -1000.0 + (j as f64)*w;
            let y0 = 0.0f64;
            let x1 = x0 + w;
            let y1 = randrange(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Box3D::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground),
            ));
        }
    }

    let mut objects: HitList = Default::default();

    objects.add(Arc::new(BVHNode::from_hitlist(&boxes1, 0.0, 1.0)));

    let light = DiffuseLight::new(SolidColor::from_rgb(7.0, 7.0, 7.0));
    objects.add(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::from_color(Color::new(0.7, 0.3, 0.1));
    objects.add(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material));

    objects.add(Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, Dielectric::new(1.5)));
    objects.add(Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)));

    // Subsurface sphere
    let boundary = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    objects.add(Arc::clone(&boundary));
    objects.add(ConstantMedium::new(Arc::clone(&boundary), 0.2, Color::new(0.2, 0.4, 0.9)));

    // Atmosphere
    let boundary = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    objects.add(ConstantMedium::new(boundary, 0.0001, Color::new(1.0, 1.0, 1.0)));

    // Earth
    let emat = Lambertian::new(Image::new("textures/earthmap.jpg"));
    objects.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));

    // Perlin textured sphere at center
    let pertext = Perlin::new(0.1);
    objects.add(Sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, Lambertian::new(pertext)));

    let mut boxes2: HitList = Default::default();
    let white = Lambertian::from_color(Color::new(0.73, 0.73, 0.73));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Sphere::new(Point3::randrange(0.0, 165.0), 10.0, Arc::clone(&white)));
    }

    objects.add(
        Translate::new(
            RotateY::new(
                Arc::new(BVHNode::from_hitlist(&boxes2, 0.0, 1.0)),
                15.0f64.to_radians()),
            Vec3::new(-100.0, 270.0, 395.0)
    ));

    objects
}

fn render(
    world: &HitList,
    camera: &Camera,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    background: Color,
) -> Vec<f64> {
    let mut data: Vec<f64> = Vec::with_capacity(image_width*image_height*3);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rf64()) / (image_width-1) as f64;
                let v = (j as f64 + rf64()) / (image_height-1) as f64;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth, background);
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
    let aspect_ratio = 1.0;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let lookfrom: Point3;
    let lookat: Point3;
    let vfov: f64;
    let aperture: f64;
    let background: Color;

    let world: HitList = match 8 {
        1 => {
            lookfrom = Point3::new(13.0 ,2.0 ,3.0);
            lookat = Point3::new(0.0 ,0.0 ,0.0);
            vfov = 20.0;
            aperture = 0.1;
            background = Color::new(0.7, 0.8, 1.0);
            random_scene()
        }
        2 => {
            lookfrom = Point3::new(13.0 ,2.0 ,3.0);
            lookat = Point3::new(0.0 ,0.0 ,0.0);
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.7, 0.8, 1.0);
            two_spheres()
        }
        3 => {
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.7, 0.8, 1.0);
            two_perlin_spheres()
        }
        4 => {
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.7, 0.8, 1.0);
            earth()
        }
        5 => {
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.0, 0.0, 0.0);
            simple_light()
        }
        6 => {
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
            background = Color::new(0.0, 0.0, 0.0);
            cornell_box()
        }
        7 => {
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
            background = Color::new(0.0, 0.0, 0.0);
            cornell_smoke()
        }
        8 => {
            lookfrom = Point3::new(478.0, 278.0, -600.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            aperture = 0.0;
            background = Color::new(0.0, 0.0, 0.0);
            final_scene()
        }
        _ => {
            lookfrom = Point3::new(15.0, 4.0, 3.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
            background = Color::new(0.7, 0.8, 1.0);
            sphere_volume_test()
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
            println!("Starting thread #{} of {}", i+1, thread_count);
            let data = render(&worldref, &camref, image_width, image_height, samples_per_thread, max_depth, background);
            println!("Completed thread #{} of {}", i+1, thread_count);
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
