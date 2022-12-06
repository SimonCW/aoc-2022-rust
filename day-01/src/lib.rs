pub fn sum_loads(input: &str) -> Vec<u32> {
    let loads_sum: Vec<u32> = input
        .split("\n\n")
        .map(|load| load.lines().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
        .collect();
    loads_sum
}

pub fn run_p1(input: &str) -> String {
    sum_loads(input).iter().max().unwrap().to_string()
}

pub fn run_p2(input: &str) -> String {
    let mut loads_sum = sum_loads(input);
    loads_sum.sort();
    loads_sum.iter().rev().take(3).sum::<u32>().to_string()
}
