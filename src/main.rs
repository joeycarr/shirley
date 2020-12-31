
mod vec3;

use image::{ImageBuffer, RgbImage, Rgb};

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

fn main() {

    let width = 512;
    let height = 256;
    let mut data: Vec<f64> = Vec::with_capacity(width*height*3);

    for y in 0..height {
        for x in 0..width {
            data.push(x as f64/width as f64);
            data.push(y as f64/height as f64);
            data.push(0.5);
        }
    }

    imsave("out.png", width, height, data);

}
