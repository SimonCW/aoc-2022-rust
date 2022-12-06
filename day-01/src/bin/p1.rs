use day_01;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("p1: {}", day_01::run_p1(&file));
    println!("p2: {}", day_01::run_p2(&file));
}
