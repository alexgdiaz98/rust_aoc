use std::{fs::read_to_string, path::Path};

#[allow(dead_code)]
pub fn day00(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    for _ in input.split('\n') {}
    let p1: usize = 0;
    let p2: usize = 0;
    (p1.to_string(), p2.to_string())
}
