use day_05::{run_p1, run_p2};
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("p1: {}", run_p1(&file));
    println!("p2: {}", run_p2(&file));
}
