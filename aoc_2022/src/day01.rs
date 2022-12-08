use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::read_to_string;
use std::path::Path;

#[allow(dead_code)]
pub fn day01(input_path: &Path) -> (String, String) {
    let mut heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let contents: String = read_to_string(input_path).expect("Error reading file");
    for group in contents.split("\n\n") {
        let sum = group
            .split('\n')
            .map(|line| line.parse::<u32>().expect("Error parsing line"))
            .sum();
        heap.push(Reverse(sum));
        if heap.len() == 4 {
            heap.pop();
        }
    }

    let p2 = heap.pop().unwrap().0 + heap.pop().unwrap().0;
    let p1 = heap.pop().unwrap().0;

    (p1.to_string(), (p1 + p2).to_string())
}
