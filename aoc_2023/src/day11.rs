use crate::coords::Coord2D as Coord;
use itertools::Itertools;
use std::collections::HashSet;
use std::{fs::read_to_string, path::Path};

#[allow(dead_code)]
pub fn day11(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut p1 = 0;
    let mut p2 = 0;
    let mut galaxies = HashSet::<Coord>::new();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut empty_rows = (0..lines.len())
        .map(|v| v as isize)
        .collect::<HashSet<isize>>();
    let mut empty_cols = (0..lines.first().unwrap().len())
        .map(|v| v as isize)
        .collect::<HashSet<isize>>();
    lines.iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                galaxies.insert(Coord(i as isize, j as isize));
                empty_rows.remove(&(i as isize));
                empty_cols.remove(&(j as isize));
            }
        })
    });
    println!("Rows: {:?}", empty_rows);
    println!("Cols: {:?}", empty_cols);
    galaxies.iter().combinations(2).for_each(|c| {
        let [a, b]: [&Coord; 2] = c.try_into().unwrap();
        let mut distance = a.hamming_distance(b);
        let min_row = a.0.min(b.0);
        let max_row = a.0.max(b.0);
        let min_col = a.1.min(b.1);
        let max_col = a.1.max(b.1);
        let extra_rows = (min_row..max_row + 1)
            .filter(|r| empty_rows.contains(r))
            .collect::<Vec<isize>>()
            .len() as isize;
        let extra_cols = (min_col..max_col + 1)
            .filter(|c| empty_cols.contains(c))
            .collect::<Vec<isize>>()
            .len() as isize;
        p1 += distance + extra_rows + extra_cols;
        p2 += distance + 999_999 * (extra_rows + extra_cols);
    });
    (p1.to_string(), p2.to_string())
}
