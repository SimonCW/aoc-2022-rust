pub fn run_p1(input: &str) -> usize {
    42
}

pub fn run_p2(input: &str) -> usize {
    42
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "replace me
";

    #[test]
    fn test_p1() {
        let result = run_p1(TEST_INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_p2() {
        let result = run_p2(TEST_INPUT);
        assert_eq!(result, 2);
    }
}
