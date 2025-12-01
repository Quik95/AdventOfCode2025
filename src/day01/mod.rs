use crate::AoCProblem;
use nom::branch::alt;
use nom::character::complete::{char, digit1, line_ending, multispace0};
use nom::combinator::{map_res, value};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair};
use nom::{IResult, Parser};

type DialRotation = i32;

#[derive(Default)]
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

impl AoCProblem for Day01 {
    fn parse_input(&mut self, input: &str) {
        let (rem, parsed_input) = delimited(
            multispace0,
            separated_list0(line_ending, parse_line),
            multispace0,
        )
        .parse(input)
        .expect("The input is not valid");
        assert!(rem.is_empty(), "The input is not fully parsed: {}", rem);

        self.rotations = parsed_input;
    }

    fn parse_input_default(&mut self) {
        self.parse_input(PROBLEM_FILE);
    }

    fn solve_part1(&self) -> Option<String> {
        let (_, zero_count) =
            self.rotations
                .iter()
                .fold((50, 0), |(state, zero_counts), &rotation| {
                    let adjusted_rotation = if rotation.abs() > 99 {
                        (rotation.abs() % 100) * rotation.signum()
                    } else {
                        rotation
                    };

                    let next_state_temp = state + adjusted_rotation;
                    let next_state = if next_state_temp < 0 {
                        100 - next_state_temp.abs()
                    } else if next_state_temp >= 100 {
                        next_state_temp - 100
                    } else {
                        next_state_temp
                    };

                    (
                        next_state,
                        if next_state == 0 {
                            zero_counts + 1
                        } else {
                            zero_counts
                        },
                    )
                });

        Some(zero_count.to_string())
    }

    fn solve_part2(&self) -> Option<String> {
        let (_, zero_count) =
            self.rotations
                .iter()
                .fold((50, 0), |(state, zero_counts), &rotation| {
                    let (adjusted_rotation, mut n_fits) = if rotation.abs() > 99 {
                        (
                            (rotation.abs() % 100) * rotation.signum(),
                            rotation.abs() / 100,
                        )
                    } else {
                        (rotation, 0)
                    };

                    let next_state_temp = state + adjusted_rotation;
                    let next_state = if next_state_temp < 0 {
                        let tmp = 100 - next_state_temp.abs();
                        if state != 0 && tmp != 0 {
                            n_fits += 1;
                        }
                        tmp
                    } else if next_state_temp >= 100 {
                        let tmp = next_state_temp - 100;
                        if state != 0 && tmp != 0 {
                            n_fits += 1;
                        }
                        tmp
                    } else {
                        next_state_temp
                    };

                    (
                        next_state,
                        if next_state == 0 {
                            zero_counts + 1 + n_fits
                        } else {
                            zero_counts + n_fits
                        },
                    )
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
        let mut day = Day01::default();
        day.parse_input(EXAMPLE_INPUT);
        assert_eq!(day.solve_part1(), Some("3".into()));
    }

    #[test]
    fn test_example_part2() {
        let mut day = Day01::default();
        day.parse_input(EXAMPLE_INPUT);
        assert_eq!(day.solve_part2(), Some("6".into()));
    }
}