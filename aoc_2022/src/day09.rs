use std::collections::HashSet;
use std::{fs::read_to_string, path::Path};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct Coord(isize, isize);

#[allow(dead_code)]
pub fn day09(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut p1: HashSet<Coord> = HashSet::from([Coord::default()]);
    let mut p2: HashSet<Coord> = HashSet::from([Coord::default()]);
    const KNOT_COUNT: usize = 10;
    let mut knots: [Coord; KNOT_COUNT] = [Coord::default(); KNOT_COUNT];
    for line in input.split('\n') {
        let (direction_token, distance_token) = line.split_once(" ").unwrap();
        for _ in 0..distance_token.parse::<isize>().unwrap() {
            knots[0] = match direction_token {
                "L" => Coord(knots[0].0 - 1, knots[0].1),
                "R" => Coord(knots[0].0 + 1, knots[0].1),
                "U" => Coord(knots[0].0, knots[0].1 - 1),
                "D" => Coord(knots[0].0, knots[0].1 + 1),
                _ => {
                    panic!("Unknown direction token: {}", direction_token);
                }
            };
            for knot_i in 1..KNOT_COUNT {
                knots[knot_i] = match (
                    knots[knot_i - 1].0 - knots[knot_i].0,
                    knots[knot_i - 1].1 - knots[knot_i].1,
                ) {
                    (0, 2) => Coord(knots[knot_i].0, knots[knot_i].1 + 1),
                    (0, -2) => Coord(knots[knot_i].0, knots[knot_i].1 - 1),
                    (2, 0) => Coord(knots[knot_i].0 + 1, knots[knot_i].1),
                    (-2, 0) => Coord(knots[knot_i].0 - 1, knots[knot_i].1),
                    (2, 1) | (1, 2) | (2, 2) => Coord(knots[knot_i].0 + 1, knots[knot_i].1 + 1),
                    (-1, 2) | (-2, 1) | (-2, 2) => Coord(knots[knot_i].0 - 1, knots[knot_i].1 + 1),
                    (1, -2) | (2, -1) | (2, -2) => Coord(knots[knot_i].0 + 1, knots[knot_i].1 - 1),
                    (-1, -2) | (-2, -1) | (-2, -2) => {
                        Coord(knots[knot_i].0 - 1, knots[knot_i].1 - 1)
                    }
                    _ => knots[knot_i],
                };
            }
            p1.insert(knots[1]);
            p2.insert(knots[KNOT_COUNT - 1]);
        }
    }
    (p1.len().to_string(), p2.len().to_string())
}
