#![warn(nonstandard_style)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused)]

use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::process::exit;

use crate::day01::day01;
use crate::day05::day05;
use crate::day06::day06;
use crate::day07::day07;
use crate::day08::day08;
use crate::day09::day09;
use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;

mod day01;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod template;

fn main() {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() != 1 && args.len() != 2 {
        eprintln!(
            "Proper usage: 'cargo run {{day}} {{path_to_input}}' ({} arguments detected)",
            args.len()
        );
        exit(-1);
    }
    let day = args
        .get(0)
        .unwrap()
        .to_owned()
        .into_string()
        .expect("Error parsing day token.")
        .parse::<u8>()
        .expect("Error converting day token to number.");
    if day == 0 || day > 25 {
        eprintln!("Provided day is not between 1-25.");
        exit(-1);
    }
    let default_input_path = OsString::from(format!("../inputs/{:0>2}.txt", day.to_string()));
    let input_path = Path::new(args.get(1).unwrap_or(&default_input_path));
    let (p1, p2) = match day {
        1 => day01(input_path),
        5 => day05(input_path),
        6 => day06(input_path),
        7 => day07(input_path),
        8 => day08(input_path),
        9 => day09(input_path),
        10 => day10(input_path),
        11 => day11(input_path),
        12 => day12(input_path),
        _ => {
            eprintln!("Invalid day: {}", day);
            exit(-1);
        }
    };
    println!("{}: p1: {} p2: {}", day, p1, p2);
}
