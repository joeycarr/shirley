
mod vec3;

fn main() {
    let v = vec3::Vec3::new(1.0, 2.0, 3.0);
    println!("Hello, world! {}", v.length().to_string());
    println!("vec: {}", v);
}
