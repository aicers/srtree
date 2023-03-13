use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/*
This is the free version of World Cities Dataset that is licensed under Creative Commons Attribution 4.0.
It contains about 43 thousand city records (population, country, location etc.).
See more about the license: https://creativecommons.org/licenses/by/4.0/
Link to the dataset: https://simplemaps.com/data/world-cities.

This function doesn't modify the dataset but only uses locations (latitude & longitude) for benchmarking purposes.
*/
pub fn world_cities_dataset() -> Vec<[f64; 2]> {
    let mut pts = Vec::new();
    let file = File::open("worldcities.csv");
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
            let mut location = [f64::INFINITY, f64::INFINITY];
            let line = line.as_ref().unwrap();
            for (i, val) in line.split(",").enumerate() {
                let mut chars = val.chars();
                chars.next();
                chars.next_back();
                let val = chars.as_str();
                if i == 2 || i == 3 {
                    let c: f64 = val.parse().unwrap_or(f64::INFINITY);
                    location[i - 2] = c;
                }
            }
            if location[0] != f64::INFINITY && location[1] != f64::INFINITY {
                pts.push(location);
            }
        }
    }

    pts
}
