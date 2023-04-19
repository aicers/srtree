use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

pub fn clustered_dataset<const D: usize>(filename: &str, skip_header: bool, skip_row: bool) -> Vec<[f64; D]> {
    let mut pts = Vec::new();
    let file = File::open(filename);
    if file.is_err() {
        return pts;
    }

    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        if skip_header {
            skip_header = false;
            continue;
        }
        if line.is_ok() {
            let mut point = [f64::INFINITY; D];
            let line = line.as_ref().unwrap();
            for (i, val) in line.split(",").enumerate() {
                if skip_row {
                    skip_row = false;
                    continue;
                }
                let chars = val.chars();
                let val = chars.as_str();
                let c: f64 = val.parse().unwrap_or(f64::INFINITY);
                point[i] = c;
            }
            if !point.contains(&f64::INFINITY) {
                pts.push(point);
            }
        }
    }
    pts
}

pub fn dns_dataset() -> Vec<[f64; 24]> {
    clustered_dataset("benches/clustered/dns.csv", false, false)
}

pub fn audio_dataset() -> Vec<[f64; 40]> {
    clustered_dataset("benches/clustered/audio.csv", false, false)
}

pub fn glove50D_dataset() -> Vec<[f64; 50]> {
    clustered_dataset("benches/clustered/glove50D.csv", true, true)
}

pub fn glove100D_dataset() -> Vec<[f64; 100]> {
    clustered_dataset("benches/clustered/glove100D.csv", true, true)
}
