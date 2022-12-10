use std::{fs::read_to_string, path::Path};

fn clock(mut p1: i64, crt: &mut [[&str; 40]; 6], x: i64, mut cycle: usize) -> (i64, usize) {
    if [-1, 0, 1].contains(&((cycle as i64 % 40) - x)) {
        crt[cycle / 40][cycle % 40] = "█";
    }
    cycle += 1;
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        p1 += x * cycle as i64;
    }
    (p1, cycle)
}

#[allow(dead_code)]
pub fn day10(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let mut x: i64 = 1;
    let mut cycle: usize = 0;
    let mut p1: i64 = 0;
    let mut crt: [[&str; 40]; 6] = [["."; 40]; 6];
    for line in input.split('\n') {
        let (opcode, val) = line.split_once(" ").unwrap_or((line, ""));
        match opcode {
            "noop" => {
                (p1, cycle) = clock(p1, &mut crt, x, cycle);
            }
            "addx" => {
                let addend = val.parse::<i64>().unwrap();
                for _ in 0..2 {
                    (p1, cycle) = clock(p1, &mut crt, x, cycle);
                }
                x += addend;
            }
            _ => {}
        }
    }
    let mut p2 = String::from("\n");
    p2.push_str(&crt.map(|row| row.join("")).join("\n"));
    (p1.to_string(), p2)
}
