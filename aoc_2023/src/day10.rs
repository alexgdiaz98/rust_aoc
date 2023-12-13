use std::collections::HashMap;
use std::{fs::read_to_string, path::Path};

use crate::coords::Coord2D as Coord;

const LEFT: Coord = Coord(-1, 0);
const RIGHT: Coord = Coord(1, 0);
const UP: Coord = Coord(0, -1);
const DOWN: Coord = Coord(0, 1);

fn next_direction(
    pipes: &HashMap<Coord, char>,
    src: Coord,
    dir: Coord,
    starting_dir: Coord,
) -> Coord {
    let new_position: char = pipes[&(src + dir)];
    match (dir, new_position) {
        (LEFT, 'L') | (RIGHT, 'J') | (UP, '|') => UP,
        (LEFT, '-') | (UP, '7') | (DOWN, 'J') => LEFT,
        (LEFT, 'F') | (RIGHT, '7') | (DOWN, '|') => DOWN,
        (RIGHT, '-') | (UP, 'F') | (DOWN, 'L') => RIGHT,
        (_, 'S') => starting_dir,
        _ => panic!(),
    }
}

fn get_start_pipe(dir_out: &Coord, dir_in: &Coord) -> char {
    match (*dir_in, *dir_out) {
        (LEFT, UP) | (DOWN, RIGHT) => 'L',
        (LEFT, LEFT) | (RIGHT, RIGHT) => '-',
        (LEFT, DOWN) | (UP, RIGHT) => 'F',
        (RIGHT, UP) | (DOWN, LEFT) => 'J',
        (RIGHT, DOWN) | (UP, LEFT) => '7',
        (UP, UP) | (DOWN, DOWN) => '|',
        _ => panic!("{} {}", dir_in, dir_out),
    }
}

fn is_inside(pipes: &HashMap<Coord, char>, space: &Coord) -> bool {
    String::from_iter(
        (0..space.0)
            .map(|x| pipes.get(&Coord(x, space.1)).unwrap_or(&'-'))
            .filter(|&c| *c != '-'),
    )
    .replacen("F7", "", usize::MAX)
    .replacen("LJ", "", usize::MAX)
    .replacen("FJ", "|", usize::MAX)
    .replacen("L7", "|", usize::MAX)
    .len()
        % 2
        == 1
}

#[allow(dead_code)]
pub fn day10(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let p1: usize;
    let mut start = Coord(0, 0);
    let mut pipes = HashMap::<Coord, char>::new();
    let mut pipes_p2 = HashMap::<Coord, char>::new();
    for (y, line) in input.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord(x as isize, y as isize);
            pipes.insert(coord, c);
            if c == 'S' {
                start = coord;
            }
        }
    }
    let starting_dir = if vec!['-', 'F', 'L'].contains(&pipes[&(start + LEFT)]) {
        LEFT
    } else if vec!['-', 'J', '7'].contains(&pipes[&(start + RIGHT)]) {
        RIGHT
    } else if vec!['|', '7', 'F'].contains(&pipes[&(start + UP)]) {
        UP
    } else {
        DOWN
    };
    let mut direction = starting_dir.clone();
    let mut current_position = start;
    let mut i = 0;
    loop {
        pipes_p2.insert(current_position, pipes[&current_position]);
        let next_direction = next_direction(&pipes, current_position, direction, starting_dir);
        current_position += direction;
        let last_dir = direction;
        direction = next_direction;
        i += 1;
        if current_position == start {
            p1 = i / 2;
            pipes_p2.insert(start, get_start_pipe(&starting_dir, &last_dir));
            break;
        }
    }
    let p2 = pipes
        .keys()
        .filter(|&space| !pipes_p2.contains_key(space)) // Only spaces
        .filter(|&space| is_inside(&pipes_p2, space))
        .collect::<Vec<_>>()
        .len();
    (p1.to_string(), p2.to_string())
}
