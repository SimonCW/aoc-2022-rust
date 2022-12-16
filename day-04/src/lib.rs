use nom::{
    bytes::complete::tag, character, multi::separated_list1, sequence::separated_pair, IResult,
};
use std::ops::RangeInclusive;

fn parse_section(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = character::complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = character::complete::u32(input)?;
    Ok((input, start..=end))
}

fn parse_section_pair(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, (section_1, section_2)) =
        separated_pair(parse_section, tag(","), parse_section)(input)?;
    Ok((input, (section_1, section_2)))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, section_pairs) =
        separated_list1(character::complete::newline, parse_section_pair)(input)?;
    Ok((input, section_pairs))
}

pub fn run_p1(input: &str) -> usize {
    let (_, sections) = parse_lines(input).unwrap();
    let mut num_fully_contained = 0;
    for (section1, section2) in sections {
        let section1_in_section2 =
            section2.contains(section1.start()) && section2.contains(section1.end());
        let section2_in_section1 =
            section1.contains(section2.start()) && section1.contains(section2.end());
        if section1_in_section2 || section2_in_section1 {
            num_fully_contained += 1
        } else {
            continue;
        }
    }
    num_fully_contained
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_p1() {
        let result = run_p1(TEST_INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_p2() {
        let result = run_p2(TEST_INPUT);
        assert_eq!(result, 4);
    }
}
