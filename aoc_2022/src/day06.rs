use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn solve(input: &str, marker_size: usize) -> Option<String> {
    let mut marker: Vec<u8> = vec![];
    let mut marker_map: HashMap<u8, usize> = HashMap::new();
    for (i, c) in input.chars().enumerate() {
        // shift the new char into the marker
        marker.insert(0, c as u8);
        marker_map
            .entry(c as u8)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        // shift the oldest char out of the marker
        if marker.len() == marker_size + 1 {
            let popped = marker.pop().unwrap();
            marker_map.entry(popped).and_modify(|count| *count -= 1);
            if *marker_map.get(&popped).unwrap() == 0 {
                marker_map.remove(&popped);
            }
        }
        // when the size of the set is the size of the marker, each value is unique
        if marker_map.keys().len() == marker_size {
            return Some((i + 1).to_string());
        }
    }
    // unreachable, assuming the input is valid
    None
}

#[allow(dead_code)]
pub fn day06(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    (
        solve(&input, 4).unwrap_or(String::from("p1 unsolved")),
        solve(&input, 14).unwrap_or(String::from("p2 unsolved")),
    )
}
