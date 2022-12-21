use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{self, delimited, preceded},
    IResult,
};

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;
    let result = match c {
        "   " => None,
        val => Some(val),
    };
    Ok((input, result))
}

fn parse_crates_line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, crates) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, crates))
}

#[derive(Debug)]
struct CraneOp {
    qty: u32,
    from: u32,
    to: u32,
}

fn parse_craneop(input: &str) -> IResult<&str, CraneOp> {
    let (input, (qty, from, to)) = sequence::tuple((
        preceded(tag("move "), complete::u32),
        preceded(tag(" from "), complete::u32),
        preceded(tag(" to "), complete::u32),
    ))(input)?;
    Ok((
        input,
        CraneOp {
            qty,
            from: from - 1,
            to: to - 1,
        },
    ))
}

fn parse_full(input: &str) -> IResult<&str, (Vec<Vec<Option<&str>>>, Vec<CraneOp>)> {
    let (input, crates_lines) = separated_list1(newline, parse_crates_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = many1(preceded(space1, digit1))(input)?;
    let (input, _) = many1(multispace1)(input)?;
    let (input, ops) = separated_list1(newline, parse_craneop)(input)?;
    Ok((input, (crates_lines, ops)))
}

// https://fasterthanli.me/series/advent-of-code-2022/part-5
fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn run_p1(input: &str) -> String {
    let (_, (crates_horizontal, ops)) = parse_full(input).unwrap();
    let mut stacks = transpose_rev(crates_horizontal);
    for CraneOp { qty, from, to } in ops {
        for _ in 0..qty {
            if let Some(popped) = stacks[from as usize].pop() {
                stacks[to as usize].push(popped);
            }
        }
    }
    let result: String = stacks
        .iter()
        .map(|vec| match vec.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();
    result
}
pub fn run_p2(input: &str) -> String {
    let (_, (crates_horizontal, ops)) = parse_full(input).unwrap();
    let mut stacks = transpose_rev(crates_horizontal);
    for CraneOp { qty, from, to } in ops {
        let idx_remove = stacks[from as usize].len() - qty as usize;
        let popped: Vec<&str> = stacks[from as usize].drain(idx_remove..).collect();
        stacks[to as usize].extend(popped);
    }
    let result: String = stacks
        .iter()
        .map(|vec| match vec.iter().last() {
            Some(c) => c,
            None => "",
        })
        .collect();
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_p1() {
        let result = run_p1(TEST_INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_p2() {
        let result = run_p2(TEST_INPUT);
        assert_eq!(result, "MCD");
    }
}
