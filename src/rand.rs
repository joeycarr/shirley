use rand;
use rand::Rng;

pub fn rf64() -> f64 { rand::random::<f64>() }

// The min is inclusive and the max is exclusive, as per the gen_range() documentation
pub fn randrange(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn randidx(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
