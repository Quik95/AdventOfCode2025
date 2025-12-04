mod day01;
mod day02;
mod day03;

use crate::day01::Day01;
use crate::day02::Day02;
use crate::day03::Day03;
use std::time::{Duration, Instant};

fn main() {
    let days: Vec<fn() -> Duration> = vec![
        run_day::<Day01>,
        run_day::<Day02>,
        run_day::<Day03>,
    ];

    let n_days = days.len();
    let total_duration = days.into_iter().fold(Duration::from_secs(0), |acc, runner| {
        acc + runner()
    });

    println!("Total time for {n_days} days: {total_duration:?}");
}

fn run_day<T: AoCProblem + Default>() -> Duration {
    let start_parse = Instant::now();
    let day_instance = T::default();
    let elapsed_parse = start_parse.elapsed();

    println!("{}:", day_instance.day_name());
    println!("\tInput parsing took: {:?}", elapsed_parse);

    let start_p1 = Instant::now();
    let mut elapsed_part1 = Duration::default();
    if let Some(part_1) = day_instance.solve_part1() {
        elapsed_part1 = start_p1.elapsed();
        println!("\tPart 1: {}", part_1);
        println!("\tPart 1 took: {:?}", elapsed_part1);
    }

    let start_p2 = Instant::now();
    let mut elapsed_part2 = Duration::default();
    if let Some(part_2) = day_instance.solve_part2() {
        elapsed_part2 = start_p2.elapsed();
        println!("\tPart 2: {}", part_2);
        println!("\tPart 2 took: {:?}", elapsed_part2);
    }

    let day_total = elapsed_parse + elapsed_part1 + elapsed_part2;
    println!("\tTotal time: {:?}\n", day_total);

    day_total
}

pub trait AoCProblem {
    fn solve_part1(&self) -> Option<String>;
    fn solve_part2(&self) -> Option<String>;
    fn day_name(&self) -> &'static str;
}
