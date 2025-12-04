use crate::AoCProblem;
use nom::character::complete::{line_ending, multispace0, satisfy};
use nom::combinator::map;
use nom::multi::{many1, separated_list0};
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser;

type BatteryJoltage = u8;

#[derive(Debug)]
pub struct Day03 {
    banks: Vec<BatteryBank>,
}

impl Day03 {
    fn new(input: &str) -> Self {
        let (rem, lines) = delimited(
            multispace0,
            separated_list0(line_ending, BatteryBank::parse_battery_bank),
            multispace0,
        )
        .parse(input)
        .expect("The input is not valid");

        assert!(rem.is_empty(), "The input was not fully parsed: {rem}");
        Day03 { banks: lines }
    }
}

#[derive(Debug)]
struct BatteryBank {
    joltages: Vec<BatteryJoltage>,
}

impl BatteryBank {
    fn parse_single_joltage(input: &str) -> IResult<&str, BatteryJoltage> {
        map(satisfy(|c| c.is_ascii_digit()), |c| {
            c.to_digit(10).unwrap() as BatteryJoltage
        })
        .parse(input)
    }

    fn parse_battery_bank(input: &str) -> IResult<&str, Self> {
        map(
            many1(Self::parse_single_joltage),
            |joltage_list: Vec<BatteryJoltage>| BatteryBank { joltages: joltage_list },
        )
        .parse(input)
    }

    fn find_largest_possible_joltage(&self, result_length: usize) -> u64 {
        let count = self.joltages.len();
        assert!(count > result_length);

        let root_layer = &self.joltages;
        let mut prev_layer = vec![0; count];
        let mut working_layer = vec![0; count-1];

        // Build the first layer based on the root layer
        prev_layer[count - 1] = root_layer[count - 1] as u64;
        for i in (0..count - 1).rev() {
            prev_layer[i] = u64::max(root_layer[i] as u64, prev_layer[i + 1]);
        }

        // Dynamic programming loop
        for layer_index in 1..result_length {
            let max_index = count - layer_index - 1;
            let power = 10u64.pow(layer_index as u32);
            working_layer[max_index] = root_layer[max_index] as u64 * power + prev_layer[max_index + 1];

            for i in (0..max_index).rev() {
                working_layer[i] = u64::max(root_layer[i] as u64 * power + prev_layer[i + 1], working_layer[i + 1]);
            }

            prev_layer[..(max_index + 1)].copy_from_slice(&working_layer[..(max_index + 1)]);
        }

        prev_layer[0]
    }
}

const PROBLEM_INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day03/actual.txt"));
impl Default for Day03 {
    fn default() -> Self {
        Self::new(PROBLEM_INPUT)
    }
}

impl AoCProblem for Day03 {
    fn solve_part1(&self) -> Option<String> {
        let result = self
            .banks
            .iter()
            .map(|bank| bank.find_largest_possible_joltage(2))
            .sum::<u64>();

        Some(result.to_string())
    }

    fn solve_part2(&self) -> Option<String> {
        let result = self
            .banks
            .iter()
            .map(|bank| bank.find_largest_possible_joltage(12))
            .sum::<u64>();

        Some(result.to_string())
    }

    fn day_name(&self) -> &'static str {
        "Day 3: Lobby"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE_INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day03/example.txt"));
    #[test]
    fn parses_input() {
        let day = Day03::new(EXAMPLE_INPUT);
        assert_eq!(day.banks.len(), 4);

        assert_eq!(day.banks[0].joltages, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(day.banks[1].joltages, vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]);
        assert_eq!(day.banks[2].joltages, vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]);
        assert_eq!(day.banks[3].joltages, vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]);
    }

    #[test]
    fn solves_part1() {
        let day = Day03::new(EXAMPLE_INPUT);
        assert_eq!(day.solve_part1(), Some("357".to_string()));
    }

    #[test]
    fn solves_part2() {
        let day = Day03::new(EXAMPLE_INPUT);
        assert_eq!(day.solve_part2(), Some("3121910778619".to_string()));
    }
}
