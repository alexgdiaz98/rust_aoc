use std::collections::{HashMap, HashSet};
use std::{fs::read_to_string, path::Path};

#[allow(dead_code)]
pub fn day04(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    // copies[card number] -> # of copies
    let mut copies = HashMap::<usize, usize>::new();
    for (line_num, line) in input.split("\n").enumerate() {
        let mut chunks = line[9..]
            .split('|')
            .map(|chunk| chunk.trim().split_whitespace().collect::<HashSet<&str>>());
        let matches = chunks
            .next()
            .unwrap()
            .intersection(&chunks.next().unwrap())
            .collect::<Vec<_>>()
            .len();
        p1 += matches
            .checked_sub(1)
            .map(|m| 2_usize.pow(m.try_into().unwrap()))
            .unwrap_or(0);
        let num_copies = *copies.entry(line_num + 1).or_insert(1);
        p2 += 1 + num_copies * matches;
        for _ in 0..num_copies {
            for index in (line_num + 2)..(line_num + 2 + matches) {
                let existing_copies = copies.get(&index).unwrap_or(&1);
                copies.insert(index, existing_copies + 1);
            }
        }
    }
    (p1.to_string(), p2.to_string())
}
