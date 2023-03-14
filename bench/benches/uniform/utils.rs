use rand::{rngs::StdRng, Rng, SeedableRng};

const INPUT_SEED: [u8; 32] = *b"PiH6Xi3GBBXhTK6UsXJYngHaF3fx4aYS";

pub fn euclidean_squared(point1: &[f64], point2: &[f64]) -> f64 {
    if point1.len() != point2.len() {
        return f64::INFINITY;
    }
    let mut distance = 0.;
    for i in 0..point1.len() {
        distance += (point1[i] - point2[i]).powi(2);
    }
    distance
}

pub fn uniform_dataset<const D: usize>(n: usize) -> Vec<[f64; D]> {
    let mut rng = StdRng::from_seed(INPUT_SEED);
    let mut pts = Vec::new();
    for _ in 0..n {
        let mut point = [0.; D];
        for item in point.iter_mut().take(D) {
            *item = rng.gen::<f64>() * 1_000_000.;
        }
        pts.push(point);
    }
    pts
}
