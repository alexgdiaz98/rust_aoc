use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, path::Path};

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut max = a;
    let mut min = b;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

#[allow(dead_code)]
pub fn day08(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let (instructions, mappings) = input.split_once("\n\n").unwrap();
    let instructions = instructions.trim().chars().collect::<Vec<char>>();
    let mut paths = HashMap::<&str, (&str, &str)>::new();
    let mut encounters = HashMap::<&str, usize>::new();
    let mut traversers = HashMap::<&str, &str>::new();
    let mut unfinished = HashSet::<&str>::new();
    let mut num_tracks = 0;
    for path in mappings.trim().split("\n") {
        let source = &path[0..3];
        let l = &path[7..10];
        let r = &path[12..15];
        paths.insert(source, (l, r));
        if source.chars().last().unwrap() == 'A' {
            traversers.insert(source, source);
            unfinished.insert(source);
            num_tracks += 1;
        }
    }

    let mut i = 0;
    while encounters.len() < num_tracks {
        let dir = instructions.get(i % instructions.len()).unwrap();
        for track in unfinished.clone().iter() {
            traversers.entry(track).and_modify(|t| {
                let split = *paths.get(t).unwrap();
                *t = match dir {
                    'L' => split.0,
                    'R' => split.1,
                    _ => panic!(""),
                }
            });
            if traversers[track].chars().last().unwrap() == 'Z' {
                encounters.insert(track, i + 1);
            }
        }
        i += 1;
    }
    let p1 = encounters["AAA"];
    let p2 = encounters
        .values()
        .copied()
        .reduce(|acc, v| lcm(acc, v))
        .unwrap();
    (p1.to_string(), p2.to_string())
}
