use std::{ffi::OsString, fs::read_to_string};

#[allow(dead_code)]
pub fn day00(input_path: &OsString) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    for line in input.split('\n') {
        let mut _tokens = line.split_whitespace();
    }
    let p1: usize = 0;
    let p2: usize = 0;
    (p1.to_string(), p2.to_string())
}
