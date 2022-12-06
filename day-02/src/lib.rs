#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    Win,
    Draw,
    Lose,
}

impl From<char> for GameResult {
    fn from(c: char) -> Self {
        match c {
            'X' => GameResult::Lose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => unimplemented!(),
        }
    }
}

impl GameResult {
    fn to_value(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => unimplemented!(),
        }
    }
}

impl Shape {
    fn combat(&self, &other: &Shape) -> GameResult {
        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => GameResult::Win,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => GameResult::Draw,
            _ => GameResult::Lose,
        }
    }
}

pub fn chose_our_shape_to_match_result(other: Shape, &desired_result: &GameResult) -> Shape {
    match (other, desired_result) {
        (Shape::Rock, GameResult::Win)
        | (Shape::Paper, GameResult::Draw)
        | (Shape::Scissors, GameResult::Lose) => Shape::Paper,
        (Shape::Scissors, GameResult::Win)
        | (Shape::Rock, GameResult::Draw)
        | (Shape::Paper, GameResult::Lose) => Shape::Rock,
        _ => Shape::Scissors,
    }
}

impl Shape {
    fn to_value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub fn run_p1(input: &str) -> usize {
    let mut total_score: usize = 0;
    for l in input.lines() {
        let (other, us) = l.split_once(' ').unwrap();
        let (other, us) = (
            Shape::from(other.chars().next().unwrap()),
            Shape::from(us.chars().next().unwrap()),
        );
        let result = us.combat(&other);
        let score = us.to_value() + result.to_value();
        total_score += score;
        // println!(
        //     "o: {:#?}, u: {:#?}, Result: {:#?}, Score: {:#?}",
        //     other, us, result, score
        // )
    }
    total_score
}

pub fn run_p2(input: &str) -> usize {
    let mut total_score: usize = 0;
    for l in input.lines() {
        let (other, result) = l.split_once(' ').unwrap();
        let (other, result) = (
            Shape::from(other.chars().next().unwrap()),
            GameResult::from(result.chars().next().unwrap()),
        );
        let us = chose_our_shape_to_match_result(other, &result);
        let score = us.to_value() + result.to_value();
        total_score += score;
        // println!(
        //     "o: {:#?}, u: {:#?}, Result: {:#?}, Score: {:#?}",
        //     other, us, result, score
        // )
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let test_string = "A Y\nB X\nC Z";
        let result = run_p1(test_string);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_p2() {
        let test_string = "A Y\nB X\nC Z";
        let result = run_p2(test_string);
        assert_eq!(result, 12);
    }
}
