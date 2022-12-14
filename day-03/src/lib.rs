use std::collections::HashMap;

fn get_char_to_value() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>()
}

pub fn run_p1(input: &str) -> usize {
    let char_to_value = get_char_to_value();

    let mut prio_sum: usize = 0;
    for rucksack in input.lines() {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        let dupe = left.chars().find(|c| right.contains(*c));
        if let Some(c) = dupe {
            prio_sum += char_to_value.get(&c).unwrap()
        } else {
            println!("{:#?}", rucksack)
        }
    }
    prio_sum
}

pub fn run_p2(input: &str) -> usize {
    let char_to_value = get_char_to_value();
    let mut prio_sum: usize = 0;
    let elves: Vec<&str> = input.lines().collect();
    for group in elves.chunks_exact(3) {
        let mut common_char: Option<char> = None;
        let e1 = group[0];
        let e2 = group[1];
        let e3 = group[2];
        for c in e1.chars() {
            if e2.contains(c) && e3.contains(c) {
                common_char = Some(c);
                break;
            };
        }
        prio_sum += char_to_value.get(&common_char.unwrap()).unwrap();
    }
    prio_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_p1() {
        let actual = run_p1(TEST_INPUT);
        assert_eq!(actual, 157);
    }

    #[test]
    fn test_p2() {
        let actual = run_p2(TEST_INPUT);
        assert_eq!(actual, 70);
    }
}
