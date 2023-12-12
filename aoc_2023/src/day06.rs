use std::{fs::read_to_string, path::Path};

fn quadratic(b: f64, c: f64) -> f64 {
    // a = -1
    let d = f64::sqrt(b.powf(2_f64) + 4_f64 * c);
    (((-b + d) / -2_f64).ceil() - ((-b - d) / -2_f64).floor()).abs() + 1_f64
}
#[allow(dead_code)]
pub fn day06(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut lines = input
        .split("\n")
        .map(|line| line.split_once(":").unwrap().1);
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();
    let t1 = times
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<u32>>();
    let d1 = distances
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<u32>>();
    let b2 = times
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<f64>()
        .unwrap();
    let c2 = distances
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<f64>()
        .unwrap() * -1_f64 - 0.5_f64;
    let p1: f64 = t1
        .iter()
        .zip(d1.iter())
        .map(|(b, c)| {
            let b_f = f64::from(*b);
            let c_f = f64::from(*c) * -1_f64 - 0.5_f64;
            quadratic(b_f, c_f)
        })
        .collect::<Vec<_>>()
        .iter()
        .product();
    let p2: f64 = quadratic(b2, c2);
    (p1.to_string(), p2.to_string())
}
