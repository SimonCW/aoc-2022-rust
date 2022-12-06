use day_02;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("p1: {}", day_02::run_p1(&file));
    println!("p2: {}", day_02::run_p2(&file));
}
