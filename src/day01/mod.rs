use crate::AoCProblem;
use nom::branch::alt;
use nom::character::complete::{char, digit1, line_ending, multispace0};
use nom::combinator::{map_res, value};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair};
use nom::{IResult, Parser};

type DialRotation = i32;

pub struct Day01 {
    pub rotations: Vec<DialRotation>,
}

fn parse_sign(input: &str) -> IResult<&str, i32> {
    alt((value(-1, char('L')), value(1, char('R')))).parse(input)
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>()).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, DialRotation> {
    pair(parse_sign, parse_number)
        .map(|(sign, number)| sign * number)
        .parse(input)
}

const PROBLEM_FILE: &str = include_str!("inputs/actual.txt");

impl Day01 {
    pub fn new(input: &str) -> Self {
        let (rem, parsed_input) = delimited(
            multispace0,
            separated_list0(line_ending, parse_line),
            multispace0,
        )
        .parse(input)
        .expect("The input is not valid");
        assert!(rem.is_empty(), "The input is not fully parsed: {}", rem);

        Self {
            rotations: parsed_input,
        }
    }
}

impl Default for Day01 {
    fn default() -> Self {
        Self::new(PROBLEM_FILE)
    }
}

impl AoCProblem for Day01 {
    fn solve_part1(&self) -> Option<String> {
        let (_, zero_count) = self.rotations.iter().fold((50, 0), |(pos, count), &rot| {
            let next_pos = (pos + rot).rem_euclid(100);
            (next_pos, count + if next_pos == 0 { 1 } else { 0 })
        });

        Some(zero_count.to_string())
    }

    fn solve_part2(&self) -> Option<String> {
        let (_, zero_count) = self.rotations.iter().fold((50, 0), |(pos, count), &rot| {
            let passed = if rot > 0 {
                (pos + rot).div_euclid(100)
            } else {
                (pos + 99).div_euclid(100) - (pos + rot + 99).div_euclid(100)
            };

            let next_pos = (pos + rot).rem_euclid(100);
            (next_pos, count + passed)
        });

        Some(zero_count.to_string())
    }

    fn day_name(&self) -> &'static str {
        "Day 1: Secret Entrance"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE_INPUT: &str = include_str!("inputs/example.txt");

    #[test]
    fn test_example_part1() {
        let day = Day01::new(EXAMPLE_INPUT);
        assert_eq!(day.solve_part1(), Some("3".into()));
    }

    #[test]
    fn test_example_part2() {
        let day = Day01::new(EXAMPLE_INPUT);
        assert_eq!(day.solve_part2(), Some("6".into()));
    }
}
