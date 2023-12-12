use std::{fs::read_to_string, path::Path};

fn game_valid(line: &str) -> bool {
    let games = line.split(':').last().unwrap().trim().split(';');
    for game in games {
        let mut red = 12;
        let mut green = 13;
        let mut blue = 14;
        for pull in game.trim().split(',') {
            let (num_str, color) = pull.trim().split_once(' ').unwrap();
            let num = i32::from_str_radix(num_str, 10).unwrap();
            match color {
                "red" => {
                    red -= num;
                    if red < 0 {
                        return false;
                    }
                },
                "green" => {
                    green -= num;
                    if green < 0 {
                        return false;
                    }
                },
                "blue" => {
                    blue -= num;
                    if blue < 0 {
                        return false;
                    }
                },
                _ => panic!("unknown color {}", color)
            }
        }
    }
    true
}

fn game_power(line: &str) -> usize {
    let games = line.split(':').last().unwrap().trim().split(';');
    let mut max_red: usize = 0;
    let mut max_green: usize = 0;
    let mut max_blue: usize = 0;
    for game in games {
        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;
        for pull in game.trim().split(',') {
            let (num_str, color) = pull.trim().split_once(' ').unwrap();
            let num = usize::from_str_radix(num_str, 10).unwrap();
            match color {
                "red" => {
                    red += num;
                },
                "green" => {
                    green += num;
                },
                "blue" => {
                    blue += num;
                },
                _ => panic!("unknown color {}", color)
            }
        }
        max_red = usize::max(max_red, red);
        max_green = usize::max(max_green, green);
        max_blue = usize::max(max_blue, blue);
    }
    max_red * max_green * max_blue
}

#[allow(dead_code)]
pub fn day02(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    for (i, line) in input.split('\n').enumerate() {
        if game_valid(line) {
            p1 += i + 1;
        }
        p2 += game_power(line);
    }
    (p1.to_string(), p2.to_string())
}
