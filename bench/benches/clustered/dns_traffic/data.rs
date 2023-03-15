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

pub fn dns_traffic_dataset() -> Vec<[f64; 24]> {
    let mut pts = Vec::new();
    let file = File::open("dns_timeseries.csv");
    if file.is_err() {
        return pts;
    }

    let reader = BufReader::new(file.unwrap());
    let mut skip_csv_header = true;
    for line in reader.lines() {
        if skip_csv_header {
            skip_csv_header = false;
            continue;
        }
        if line.is_ok() {
            let mut point = [f64::INFINITY; 24];
            let line = line.as_ref().unwrap();
            for (i, val) in line.split(",").enumerate() {
                let mut chars = val.chars();
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
