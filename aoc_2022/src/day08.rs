use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, path::Path};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Tree(usize, usize);

impl Tree {
    pub fn new(x: usize, y: usize) -> Self {
        Self { 0: x, 1: y }
    }
}

#[allow(dead_code)]
pub fn day08(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut trees: HashMap<Tree, u32> = HashMap::new();
    let mut p1: HashSet<Tree> = HashSet::new();
    let p2: usize = 0; // TODO: p2
    let lines = input.split('\n');
    const Y_MAX: usize = 99;
    const X_MAX: usize = 99;
    let mut max_column: [i32; X_MAX] = [-1; X_MAX];
    for (y, line) in lines.enumerate() {
        let mut max_row: i32 = -1;
        for (x, c) in line.chars().enumerate() {
            let tree: Tree = Tree::new(x, y);
            let digit = c.to_digit(10).unwrap();
            trees.insert(tree, digit);
            if digit as i32 > max_row {
                match p1.get(&tree) {
                    Some(_) => {}
                    None => {
                        p1.insert(tree);
                    }
                }
                max_row = digit as i32;
            }
            if digit as i32 > max_column[x] {
                match p1.get(&tree) {
                    Some(_) => {}
                    None => {
                        p1.insert(tree);
                    }
                }
                max_column[x] = digit as i32;
            }
        }
    }
    max_column = [-1; X_MAX];
    for y in (0..Y_MAX).rev() {
        let mut max_row: i32 = -1;
        for x in (0..X_MAX).rev() {
            let tree: Tree = Tree::new(x, y);
            let digit = *trees.get(&tree).unwrap();
            if digit as i32 > max_row {
                match p1.get(&tree) {
                    Some(_) => {}
                    None => {
                        p1.insert(tree);
                    }
                }
                max_row = digit as i32;
            }
            if digit as i32 > max_column[x] {
                match p1.get(&tree) {
                    Some(_) => {}
                    None => {
                        p1.insert(tree);
                    }
                }
                max_column[x] = digit as i32;
            }
        }
    }
    (p1.len().to_string(), p2.to_string())
}
