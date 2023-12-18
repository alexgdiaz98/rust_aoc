use crate::coords::Coord2D as Coord;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs::read_to_string, path::Path};

const LEFT: Coord = Coord(0, -1);
const RIGHT: Coord = Coord(0, 1);
const UP: Coord = Coord(-1, 0);
const DOWN: Coord = Coord(1, 0);

fn new_directions(direction: Coord, element: char) -> Vec<Coord> {
    match (element, direction) {
        ('.', _) | ('-', LEFT) | ('-', RIGHT) | ('|', UP) | ('|', DOWN) => vec![direction],
        ('|', LEFT) | ('|', RIGHT) => vec![UP, DOWN],
        ('-', UP) | ('-', DOWN) => vec![LEFT, RIGHT],
        ('/', LEFT) | ('\\', RIGHT) => vec![DOWN],
        ('\\', LEFT) | ('/', RIGHT) => vec![UP],
        ('/', UP) | ('\\', DOWN) => vec![RIGHT],
        ('\\', UP) | ('/', DOWN) => vec![LEFT],
        _ => panic!(
            "Unexpected element direction combination: {} {}",
            element, direction
        ),
    }
}

fn calculate_energized(grid: &HashMap<Coord, char>, initial_beam: (Coord, Coord)) -> usize {
    let mut energized = HashSet::<Coord>::new();
    let mut already_considered = HashSet::<(Coord, Coord)>::new();
    let mut beams = VecDeque::<(Coord, Coord)>::from([initial_beam]);
    while !beams.is_empty() {
        let (beam_coord, beam_direction) = beams.pop_front().unwrap();
        if grid.contains_key(&beam_coord) {
            energized.insert(beam_coord);
        }
        if let Some(&c) = grid.get(&beam_coord) {
            new_directions(beam_direction, c)
                .iter()
                .filter(|&beam2| !already_considered.contains(&(beam_coord + *beam2, *beam2)))
                .for_each(|&beam2| {
                    beams.push_back((beam_coord + beam2, beam2));
                });
        }
        already_considered.insert((beam_coord, beam_direction));
    }
    energized.len()
}

#[allow(dead_code)]
pub fn day16(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut grid = HashMap::<Coord, char>::new();
    let lines = input.split("\n").collect_vec();
    let height = lines.len();
    let width = lines.first().unwrap().len();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid.insert(Coord(i as isize, j as isize), c);
        }
    }
    let p1 = calculate_energized(&grid, (Coord(0, 0), RIGHT));
    let mut max_energized = 0;
    let mut max_energized_beam = (Coord(0, 0), RIGHT);
    for i in 0..height {
        let mut initial_coord = Coord(i as isize, 0);
        let mut energized = calculate_energized(&grid, (initial_coord, RIGHT));
        if energized > max_energized {
            max_energized = energized;
            max_energized_beam = (initial_coord, RIGHT);
        }
        initial_coord = Coord(i as isize, width as isize - 1);
        energized = calculate_energized(&grid, (initial_coord, LEFT));
        if energized > max_energized {
            max_energized = energized;
            max_energized_beam = (initial_coord, LEFT);
        }
    }
    for j in 0..width {
        let mut initial_coord = Coord(0, j as isize);
        let mut energized = calculate_energized(&grid, (initial_coord, DOWN));
        if energized > max_energized {
            max_energized = energized;
            max_energized_beam = (initial_coord, DOWN);
        }
        initial_coord = Coord(height as isize - 1, j as isize);
        energized = calculate_energized(&grid, (initial_coord, UP));
        if energized > max_energized {
            max_energized = energized;
            max_energized_beam = (initial_coord, UP);
        }
    }
    let p2 = max_energized;
    (p1.to_string(), p2.to_string())
}
