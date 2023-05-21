use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn world_cities_dataset() -> Vec<[f64; 2]> {
    world_cities()
}

pub fn dns_dataset() -> Vec<[f64; 24]> {
    clustered_dataset("benches/clustered/datasets/dns.csv", false)
}

pub fn home_dataset() -> Vec<[f64; 64]> {
    home_electricity_usage()
}

pub fn audio_dataset() -> Vec<[f64; 40]> {
    clustered_dataset("benches/clustered/datasets/drone_audio.csv", false)
}

pub fn glove50d_dataset() -> Vec<[f64; 50]> {
    clustered_dataset("benches/clustered/datasets/glove50D.csv", true)
}

pub fn covtype54d_dataset() -> Vec<[f64; 54]> {
    clustered_dataset("benches/clustered/datasets/covtype.csv", false)
}

pub fn glove100d_dataset() -> Vec<[f64; 100]> {
    clustered_dataset("benches/clustered/datasets/glove100D.csv", true)
}

pub fn darpa_audio_dataset() -> Vec<[f64; 192]> {
    clustered_dataset("benches/clustered/datasets/darpa_audio.csv", true)
}

fn world_cities() -> Vec<[f64; 2]> {
    let mut pts = Vec::new();
    let file = File::open("benches/clustered/datasets/worldcities.csv");
    if file.is_err() {
        return pts;
    }

    let mut skip_header = true;
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        if skip_header {
            skip_header = false;
            continue;
        }
        if line.is_ok() {
            let mut point = [f64::INFINITY; 2];
            let line = line.as_ref().unwrap();
            for (i, val) in line.split(",").enumerate() {
                let mut chars = val.chars();
                chars.next();
                chars.next_back();
                let val = chars.as_str();
                let c: f64 = val.parse().unwrap_or(f64::INFINITY);
                if i == 2 {
                    point[0] = c;
                } else if i == 3 {
                    point[1] = c;
                }
            }
            if !point.contains(&f64::INFINITY) {
                pts.push(point);
            }
        }
    }
    pts
}

fn home_electricity_usage<const D: usize>() -> Vec<[f64; D]> {
    let mut pts = Vec::new();
    let file = File::open("benches/clustered/datasets/home.csv");
    if file.is_err() {
        return pts;
    }

    let mut skip_header = true;
    let reader = BufReader::new(file.unwrap());

    let mut electricity_usage = Vec::new();
    for line in reader.lines() {
        if skip_header {
            skip_header = false;
            continue;
        }
        if line.is_ok() {
            let line = line.as_ref().unwrap();
            let columns: Vec<&str> = line.split(",").into_iter().collect();
            let usage = columns[1].parse().unwrap_or(f64::INFINITY);
            if usage != f64::INFINITY {
                electricity_usage.push(usage);
            }
        }
    }

    // Sample D dimensional points from the electricity usage sequentially
    let mut i = 0;
    while i + D < electricity_usage.len() {
        let mut point = [f64::INFINITY; D];
        for j in 0..D {
            point[j] = electricity_usage[i + j];
        }
        pts.push(point);
        i += D;
    }
    pts
}

fn clustered_dataset<const D: usize>(filename: &str, mut skip_header: bool) -> Vec<[f64; D]> {
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
                if i >= D {
                    break;
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
