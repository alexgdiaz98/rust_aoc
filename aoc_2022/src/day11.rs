use std::collections::VecDeque;
use std::{fs::read_to_string, path::Path};

#[derive(Debug)]

pub enum Operation {
    ADD,
    MULT,
}

#[derive(Debug)]
pub struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: Operation,
    pub operation_target: Option<usize>,
    pub test: usize,
    pub true_target: usize,
    pub false_target: usize,
    pub inspected: usize,
}

impl Monkey {
    fn from(monkey_input: &str) -> Option<Self> {
        let lines = monkey_input
            .split("\n")
            .map(|line| line.trim().split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let items = VecDeque::from(
            lines
                .get(1)?
                .get(2..)? // Skip "Starting items: "
                .into_iter()
                .filter_map(|item| item.replace(",", "").parse::<usize>().ok())
                .collect::<Vec<usize>>(),
        );
        let operation_tokens = lines.get(2)?.get(4..)?.to_owned();
        let test = lines.get(3)?.get(3)?.parse::<usize>().unwrap();
        let true_target = lines.get(4)?.get(5)?.parse::<usize>().unwrap();
        let false_target = lines.get(5)?.get(5)?.parse::<usize>().unwrap();
        Some(Self {
            items: items,
            operation: match *operation_tokens.get(0)? {
                "+" => Operation::ADD,
                "*" => Operation::MULT,
                _ => panic!("Unexpected operand"),
            },
            operation_target: match *operation_tokens.get(1)? {
                "old" => None,
                s => Some(s.parse::<usize>().unwrap()),
            },
            test: test,
            true_target,
            false_target,
            inspected: 0,
        })
    }
}

fn sim(input_path: &Path, p1: bool) -> usize {
    let monkey_strs = read_to_string(input_path).expect("Error reading file");
    let mut monkeys: Vec<Monkey> = vec![];
    for monkey_str in monkey_strs.split("\n\n") {
        monkeys.push(Monkey::from(monkey_str).expect("Error parsing monkey"));
    }
    let num_monkeys = monkeys.len();
    // NOTE: Each monkey has a unique prime number as its test value, so lcm is the product of all test values.
    let lcm: usize = monkeys.iter().map(|m| m.test).product();
    let turns = if p1 { 20_usize } else { 10000_usize } * num_monkeys;
    for turn in 0..turns {
        while monkeys[turn % num_monkeys].items.len() > 0 {
            let mut item = monkeys[turn % num_monkeys].items.pop_front().unwrap();
            monkeys[turn % num_monkeys].inspected += 1;
            let operand = match monkeys[turn % num_monkeys].operation_target {
                Some(x) => x,
                None => item,
            };
            item = match monkeys[turn % num_monkeys].operation {
                Operation::ADD => item + operand,
                Operation::MULT => item * operand,
            };
            item = if p1 { item / 3 } else { item % lcm };
            let target_i = if item % monkeys[turn % num_monkeys].test == 0 {
                monkeys[turn % num_monkeys].true_target
            } else {
                monkeys[turn % num_monkeys].false_target
            };
            monkeys[target_i].items.push_back(item);
        }
    }
    let mut inspection_map = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    inspection_map.sort();
    let monkey_business = inspection_map.pop().unwrap() * inspection_map.pop().unwrap();
    monkey_business
}

#[allow(dead_code)]
pub fn day11(input_path: &Path) -> (String, String) {
    let p1 = sim(input_path, true);
    let p2 = sim(input_path, false);
    (p1.to_string(), p2.to_string())
}
