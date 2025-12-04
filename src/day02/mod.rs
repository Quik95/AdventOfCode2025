use crate::AoCProblem;
use nom::character::complete::{char, digit1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{Finish, IResult, Parser};

pub struct Day02 {
    pub product_id_ranges: Vec<ProductIDRange>,
}

impl Day02 {
    pub fn new(input: &'static str) -> Self {
        let (remaining, product_id_ranges) = separated_list1(char(','), ProductIDRange::parse)
            .parse(input)
            .finish()
            .expect("Failed to parse product ID ranges from input");

        assert!(remaining.is_empty(), "The input is not fully parsed: {}", remaining);

        Self { product_id_ranges }
    }
}

const PROBLEM_INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02/actual.txt"));
impl Default for Day02 {
    fn default() -> Self {
        Self::new(PROBLEM_INPUT)
    }
}

impl AoCProblem for Day02 {
    fn solve_part1(&self) -> Option<String> {
        Some(
            self.product_id_ranges
                .iter()
                .flat_map(|range| range.low..=range.high)
                .filter(|&id| !is_symmetric(id))
                .sum::<u64>()
                .to_string(),
        )
    }

    fn solve_part2(&self) -> Option<String> {
        Some(
            self.product_id_ranges
                .iter()
                .flat_map(|range| range.low..=range.high)
                .filter(|&id| is_repeating_pattern(id))
                .sum::<u64>()
                .to_string(),
        )
    }

    fn day_name(&self) -> &'static str {
        "Day 2: Gift Shop"
    }
}

pub fn is_symmetric(id: u64) -> bool {
    let length = (id.ilog10() + 1) as u64;
    if length % 2 == 1 {
        return true;
    }

    let low_part = id % (10u64.pow((length / 2) as u32));
    let high_part = (id - low_part) / (10u64.pow((length / 2) as u32));

    low_part != high_part
}

fn is_repeating_pattern(id: u64) -> bool {
    let as_str = id.to_string();
    let doubled = as_str.repeat(2);

    doubled
        .get(1..(doubled.len() - 1))
        .expect("To be valid")
        .contains(&as_str)
}

#[derive(Debug, Eq, PartialEq)]
pub struct ProductIDRange {
    pub low: u64,
    pub high: u64,
}

impl ProductIDRange {
    pub fn parse(input: &'static str) -> IResult<&'static str, Self> {
        map(
            separated_pair(digit1, char('-'), digit1),
            |(low, high): (&str, &str)| Self {
                low: low.parse::<u64>().expect("To be a valid number"),
                high: high.parse::<u64>().expect("To be a valid number"),
            },
        )
        .parse(input)
    }
}

impl TryFrom<&'static str> for ProductIDRange {
    type Error = nom::error::Error<&'static str>;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let (_remaining, range) = nom::combinator::all_consuming(Self::parse).parse(value).finish()?;
        Ok(range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE_INPUT: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02/example.txt"));

    #[test]
    fn parses_input() {
        let day02 = Day02::new(EXAMPLE_INPUT);

        fn range(low: u64, high: u64) -> ProductIDRange {
            ProductIDRange {
                low,
                high,
            }
        }
        let expected = vec![
            range(11, 22),
            range(95, 115),
            range(998, 1012),
            range(1188511880, 1188511890),
            range(222220, 222224),
            range(1698522, 1698528),
            range(446443, 446449),
            range(38593856, 38593862),
            range(565653, 565659),
            range(824824821, 824824827),
            range(2121212118, 2121212124),
        ];
        assert_eq!(day02.product_id_ranges, expected)
    }

    #[test]
    fn detects_invalid_ids() {
        assert_eq!(is_symmetric("11".parse().unwrap()), false);
        assert_eq!(is_symmetric("22".parse().unwrap()), false);
        assert_eq!(is_symmetric("1010".parse().unwrap()), false);
        assert_eq!(is_symmetric("1188511885".parse().unwrap()), false);
        assert_eq!(is_symmetric("222222".parse().unwrap()), false);
        assert_eq!(is_symmetric("446446".parse().unwrap()), false);
        assert_eq!(is_symmetric("38593859".parse().unwrap()), false);
        assert_eq!(is_symmetric("38593862".parse().unwrap()), true);
    }

    #[test]
    fn example_part1() {
        assert_eq!(Day02::new("11-22").solve_part1(), Some("33".into()));
        assert_eq!(Day02::new("95-115").solve_part1(), Some("99".into()));

        let day02 = Day02::new(EXAMPLE_INPUT);
        assert_eq!(day02.solve_part1(), Some("1227775554".into()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(Day02::new("11-22").solve_part2(), Some("33".into()));
        assert_eq!(Day02::new("95-115").solve_part2(), Some("210".into()));

        let day02 = Day02::new(EXAMPLE_INPUT);
        assert_eq!(day02.solve_part2(), Some("4174379265".into()));
    }
}
