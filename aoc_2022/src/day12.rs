use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::isize;
use std::{fs::read_to_string, path::Path};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Coord(isize, isize, usize);

impl Coord {
    pub fn from(x: isize, y: isize) -> Self {
        Self(x, y, usize::MAX)
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.2.partial_cmp(&other.2)
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2)
    }
}

fn dijkstra(
    map: &HashMap<Coord, isize>,
    neighbors: &HashMap<Coord, Vec<Coord>>,
    start: Coord,
    end: Coord,
) -> usize {
    let mut dist: HashMap<Coord, usize> = HashMap::new();
    dist.insert(start, 0);
    let mut prev: HashMap<Coord, Option<Coord>> = HashMap::new();
    let mut Q: BinaryHeap<Reverse<Coord>> = BinaryHeap::new();
    for c in map {
        if start != *c.0 {
            dist.insert(*c.0, usize::MAX);
            prev.insert(*c.0, None);
        }
        Q.push(Reverse(Coord(c.0 .0, c.0 .1, usize::MAX)));
    }
    while Q.len() > 0 {
        let u = Q.pop().unwrap().0;
        let defaultN = Vec::<Coord>::new();
        for v in neighbors.get(&u).unwrap_or_else(|| &&defaultN) {
            let alt = u.2;
            if alt < v.2 {
                v.2 = alt;
            }
        }
    }
    *dist.get(&end).unwrap()
}

#[allow(dead_code)]
pub fn day12(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut map: HashMap<Coord, isize> = HashMap::new();
    let mut neighbors: HashMap<Coord, Vec<Coord>> = HashMap::new();
    let mut start: Coord;
    let mut end: Coord;
    for (y, line) in input.split('\n').enumerate() {
        for (x, mut c) in line.chars().enumerate() {
            let coord = Coord::from(x as isize, y as isize);
            match c {
                'S' => {
                    start = coord;
                    c = '`'; // ASCII character before 'a'
                }
                'E' => {
                    end = coord;
                    c = '{'; // ASCII character after 'z'
                }
                _ => {}
            }
            map.insert(coord, c as isize);
            for (x_off, y_off) in [(-1_isize, 0_isize), (0, -1)] {
                let n_coord = Coord::from(x as isize + x_off, y as isize + y_off);
                if map.contains_key(&n_coord) {
                    let n_val = *map.get(&n_coord).unwrap();
                    match c as isize - n_val {
                        isize::MIN..=-2 => {
                            neighbors
                                .entry(n_coord)
                                .and_modify(|e| e.push(coord))
                                .or_insert(vec![coord]);
                        }
                        2..=isize::MAX => {
                            neighbors
                                .entry(coord)
                                .and_modify(|e| e.push(n_coord))
                                .or_insert(vec![n_coord]);
                        }
                        _ => {
                            neighbors
                                .entry(n_coord)
                                .and_modify(|e| e.push(coord))
                                .or_insert(vec![coord]);
                            neighbors
                                .entry(coord)
                                .and_modify(|e| e.push(n_coord))
                                .or_insert(vec![n_coord]);
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", neighbors.get(&Coord::from(3, 1)).unwrap());

    let p1: usize = 0;
    let p2: usize = 0;
    (p1.to_string(), p2.to_string())
}
