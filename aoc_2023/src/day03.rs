use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use std::{collections::HashMap, fs::read_to_string, path::Path};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coord(isize, isize);

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

fn get_neighbors(origin: &Coord, num: isize) -> HashSet<Coord> {
    let num_size = num.to_string().len() as isize;
    let j2 = *origin + Coord(0, -1);
    let j1 = *origin - Coord(0, num_size);
    let mut neighbors = HashSet::from([
        j1 + Coord(-1, -1),
        j1 + Coord(0, -1),
        j1 + Coord(1, -1),
        j2 + Coord(-1, 1),
        j2 + Coord(0, 1),
        j2 + Coord(1, 1),
    ]);
    for k in j1.1..=j2.1 {
        neighbors.insert(Coord(origin.0 - 1, k));
        neighbors.insert(Coord(origin.0 + 1, k));
    }
    neighbors
}

#[allow(dead_code)]
pub fn day03(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut symbols = HashMap::<Coord, char>::new();
    let mut neighbors = Vec::<(isize, HashSet<Coord>)>::new();
    let mut ongoing_number: Option<isize>;
    for (i, line) in input.split("\n").enumerate() {
        ongoing_number = None;
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as isize;
                match ongoing_number {
                    Some(num) => {
                        ongoing_number = Some(num * 10 + digit);
                    }
                    None => ongoing_number = Some(digit),
                }
            } else {
                if ongoing_number.is_some() {
                    let part_num = ongoing_number.unwrap();
                    let local_neighbors = get_neighbors(&Coord(i as isize, j as isize), part_num);
                    neighbors.push((part_num, local_neighbors));
                    ongoing_number = None;
                }
                if c != '.' {
                    symbols.insert(Coord(i as isize, j as isize), c);
                    if i == 1 {
                        println!("Token {}", Coord(i as isize, j as isize));
                    }
                }
            }
        }
        if ongoing_number.is_some() {
            let part_num = ongoing_number.unwrap();
            let local_neighbors = get_neighbors(&Coord(i as isize, line.len() as isize), part_num);
            neighbors.push((part_num, local_neighbors));
        }
    }
    let mut p1 = 0;
    let mut p2 = 0;
    for part in &neighbors {
        if part.1.iter().any(|neighbor| symbols.get(neighbor).is_some()) {
            p1 += part.0;
        }
    }
    for symbol in symbols {
        let adj: Vec<isize> = neighbors
            .iter()
            .filter(|part| part.to_owned().to_owned().1.contains(&symbol.0))
            .map(|part| part.0)
            .collect();
        if adj.len() == 2 {
            p2 += adj.iter().product::<isize>();
        }
    }

    (p1.to_string(), p2.to_string())
}
