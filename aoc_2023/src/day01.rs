use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn identify_digit(s: &str, digits: &HashMap<&str, usize>) -> usize {
    let keys = digits.keys();
    let (key, _) = keys
        .map(|key| (key, s.find(key).unwrap_or(usize::MAX)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    *digits.get(key).unwrap()
}

#[allow(dead_code)]
pub fn day01(input_path: &Path) -> (String, String) {
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let digits = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let p2_digits = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let p2_rev_digits = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ]);
    let mut p1 = 0;
    let mut p2 = 0;
    for line in contents.split("\n") {
        let rev: String = line.chars().rev().collect();
        p1 += identify_digit(line, &digits) * 10;
        p1 += identify_digit(&rev, &digits);
        p2 += identify_digit(line, &p2_digits) * 10;
        p2 += identify_digit(&rev, &p2_rev_digits);
    }

    (p1.to_string(), p2.to_string())
}
