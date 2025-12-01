use nom::branch::alt;
use nom::character::complete::{char, digit1, line_ending, multispace0};
use nom::combinator::{map_res, value};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair};
use nom::{IResult, Parser};

const INPUT_FILE: &str = include_str!("../inputs/day01/actual.txt");

fn main() {
    let (rest, parsed) = parse_input_file(INPUT_FILE).expect("The input is not valid");
    assert!(rest.is_empty(), "The input is not fully parsed: {}", rest);

    let (end_state, zero_count) = parsed
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

    let (end_state2, zero_count2) =
        parsed
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

    println!("End state was: {end_state}");
    println!("Number of times counter was 0: {zero_count}");
    println!("---------");
    println!("End state was: {end_state2}");
    println!("Number of times counter was 0: {zero_count2}");
}

type DialRotation = i32;

fn parse_input_file(input: &str) -> IResult<&str, Vec<DialRotation>> {
    delimited(
        multispace0,
        separated_list0(line_ending, parse_line),
        multispace0,
    )
    .parse(input)
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
