use itertools::Itertools;
use std::collections::HashMap;
use std::{fs::read_to_string, path::Path};

use crate::coords::Coord2D as Coord;

const LEFT: Coord = Coord(-1, 0);
const RIGHT: Coord = Coord(1, 0);
const UP: Coord = Coord(0, -1);
const DOWN: Coord = Coord(0, 1);
const DIRECTIONS: [Coord; 4] = [LEFT, RIGHT, UP, DOWN];

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

fn horizontal_scan(
    pipes: &HashMap<Coord, char>,
    crossings: &mut isize,
    ongoing: char,
    coord: &Coord,
) -> char {
    let mut ret = ongoing;
    match pipes.get(coord) {
        Some('|') => *crossings += 1,
        Some('F') => ret = 'F',
        Some('7') => match ongoing {
            'F' => ret = '-',
            'L' => {
                ret = '_';
                *crossings += 1;
            }
            _ => panic!(),
        },
        Some('J') => match ongoing {
            'F' => {
                ret = '-';
                *crossings += 1;
            }
            'L' => ret = '-',
            _ => panic!("{}", ongoing),
        },
        Some('L') => ret = 'L',
        _ => {}
    }
    ret
}

fn vertical_scan(
    pipes: &HashMap<Coord, char>,
    crossings: &mut isize,
    ongoing: char,
    coord: &Coord,
) -> char {
    let mut ret = ongoing;
    match pipes.get(coord) {
        Some('-') => *crossings += 1,
        Some('F') => ret = 'F',
        Some('L') => match ongoing {
            'F' => ret = '|',
            '7' => {
                ret = '|';
                *crossings += 1;
            }
            _ => panic!(),
        },
        Some('J') => match ongoing {
            'F' => {
                ret = '|';
                *crossings += 1;
            }
            '7' => ret = '|',
            _ => panic!(),
        },
        Some('7') => ret = '7',
        _ => {}
    }
    ret
}

fn is_inside(pipes_p2: &HashMap<Coord, char>, space: &Coord, width: isize, height: isize) -> bool {
    let mut ongoing = '-';
    let mut crossings = 0;
    for x in 0..space.0 {
        ongoing = horizontal_scan(&pipes_p2, &mut crossings, ongoing, &Coord(x, space.1));
    }
    if crossings % 2 != 1 {
        return false;
    }
    crossings = 0;
    ongoing = '-';
    for x in (space.0 + 1)..width {
        ongoing = horizontal_scan(&pipes_p2, &mut crossings, ongoing, &Coord(x, space.1));
    }
    if crossings % 2 != 1 {
        return false;
    }
    crossings = 0;
    ongoing = '|';
    for y in 0..space.1 {
        ongoing = vertical_scan(&pipes_p2, &mut crossings, ongoing, &Coord(space.0, y));
    }
    if crossings % 2 != 1 {
        return false;
    }
    crossings = 0;
    ongoing = '|';
    for y in (space.1 + 1)..height {
        ongoing = vertical_scan(&pipes_p2, &mut crossings, ongoing, &Coord(space.0, y));
    }
    if crossings % 2 != 1 {
        return false;
    }
    true
}

#[allow(dead_code)]
pub fn day10(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    let mut start = Coord(0, 0);
    let mut pipes = HashMap::<Coord, char>::new();
    let mut pipes_p2 = HashMap::<Coord, char>::new();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let width = lines.first().unwrap().len() as isize;
    let height = lines.len() as isize;
    for (y, line) in lines.iter().enumerate() {
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
    let mut halfway_point: Option<usize> = None;
    let mut i = 0;
    loop {
        pipes_p2.insert(current_position, pipes[&current_position]);
        let next_direction = next_direction(&pipes, current_position, direction, starting_dir);
        current_position += direction;
        let last_dir = direction;
        direction = next_direction;
        i += 1;
        if current_position == start {
            halfway_point = Some(i / 2);
            i = 0;
            pipes_p2.insert(start, get_start_pipe(&starting_dir, &last_dir));
            break;
        }
    }
    p1 = halfway_point.unwrap();
    p2 = pipes
        .keys()
        .filter(|&space| !pipes_p2.contains_key(space)) // Only spaces
        .filter(|&space| is_inside(&pipes_p2, space, width, height))
        .collect::<Vec<_>>()
        .len();
    (p1.to_string(), p2.to_string())
}
