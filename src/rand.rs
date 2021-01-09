use rand;
use rand::Rng;

pub fn rf64() -> f64 { rand::random::<f64>() }

pub fn randrange(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
