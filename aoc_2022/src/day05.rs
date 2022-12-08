use std::{fs::read_to_string, path::Path};

const STACK_COUNT: usize = 9;

fn parse_stacks(stacks: &[Vec<u8>; STACK_COUNT]) -> String {
    let mut ret = String::new();
    for option in stacks.iter().map(|s| s.last()) {
        match option {
            Some(s) => ret.push_str(&((*s as char).to_string())[..]),
            None => ret.extend([" "]),
        }
    }
    ret
}

#[allow(dead_code)]
pub fn day05(input_path: &Path) -> (String, String) {
    let mut p1_stacks: [Vec<u8>; STACK_COUNT] = Default::default();
    let mut p2_stacks: [Vec<u8>; STACK_COUNT] = Default::default();
    let contents: String = read_to_string(input_path).expect("Error reading file");
    let (stack_str, instructions_str) = contents
        .split_once("\n\n")
        .expect("Error parsing input format.");
    // Parse initial stack
    for line in stack_str.lines().enumerate().filter(|line| line.0 <= 7) {
        for token in line.1.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            match token.1[1] {
                ' ' => {}
                c => {
                    p1_stacks[token.0].insert(0, c as u8);
                    p2_stacks[token.0].insert(0, c as u8);
                }
            }
        }
    }
    for line in instructions_str.lines().enumerate() {
        // Parse stack instructions
        let tokens = line.1.split_whitespace().collect::<Vec<&str>>();
        let move_n = tokens.get(1).unwrap().parse::<usize>().unwrap();
        let move_from = tokens.get(3).unwrap().parse::<usize>().unwrap();
        let move_to = tokens.get(5).unwrap().parse::<usize>().unwrap();
        let mut mover: Vec<u8> = vec![];
        for _ in 0..move_n {
            let temp = p1_stacks[move_from - 1]
                .pop()
                .expect("Popping from empty stack");
            let temp2 = p2_stacks[move_from - 1]
                .pop()
                .expect("Popping from empty stack");
            mover.insert(0, temp2);
            p1_stacks[move_to - 1].push(temp);
        }
        p2_stacks[move_to - 1].append(&mut mover);
    }

    (parse_stacks(&p1_stacks), parse_stacks(&p2_stacks))
}
