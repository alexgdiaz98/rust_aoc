use std::iter::Sum;
use std::{fs::read_to_string, path::Path};

struct Extrapolated(pub isize, pub isize);

impl Extrapolated {
    fn extrapolate(&self, next_line: &Self) -> Self {
        Self(next_line.0 - self.0, self.1 + next_line.1)
    }
}

impl Sum for Extrapolated {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Extrapolated(0, 0), |acc, t| {
            Extrapolated(acc.0 + t.0, acc.1 + t.1)
        })
    }
}

fn parse_line(tokens: &Vec<isize>) -> Extrapolated {
    if tokens.iter().all(|t| *t == 0) {
        return Extrapolated(0, 0);
    }
    let mut new_line = Vec::<isize>::new();
    let mut last = None;
    tokens.iter().for_each(|t| match last {
        None => {
            last = Some(t);
        }
        Some(t2) => {
            new_line.push(t - t2);
            last = Some(t);
        }
    });
    let first = tokens.first().unwrap();
    parse_line(&new_line).extrapolate(&Extrapolated(*first, *last.unwrap()))
}

#[allow(dead_code)]
pub fn day09(input_path: &Path) -> (String, String) {
    let input: String = read_to_string(input_path).expect("Error reading file");
    let extrapolated: Extrapolated = input
        .split("\n")
        .into_iter()
        .map(|line| {
            let tokens: Vec<isize> = line
                .trim()
                .split_whitespace()
                .map(|t| t.parse::<isize>().unwrap())
                .collect();
            parse_line(&tokens)
        })
        .sum();
    (extrapolated.1.to_string(), extrapolated.0.to_string())
}
