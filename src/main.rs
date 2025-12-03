mod day01;
mod day02;

use crate::day01::Day01;
use std::time::{Duration, Instant};
use crate::day02::Day02;

fn main() {
    let days: Vec<Box<dyn AoCProblem>> = vec![
        Box::<Day01>::default(),
        Box::<Day02>::default(),
    ];

    let n_days = days.len();
    let total_duration = days.into_iter().fold(Duration::from_secs(0), |acc, day| {
        acc + day.print_solution()
    });

    println!("Total time for {n_days} days: {total_duration:?}");
}

pub trait AoCProblem {
    fn solve_part1(&self) -> Option<String>;
    fn solve_part2(&self) -> Option<String>;
    fn day_name(&self) -> &'static str;

    fn print_solution(&self) -> Duration {
        println!("{}:", self.day_name());

        let start = Instant::now();
        let mut elapsed_part1 = Duration::default();
        if let Some(part_1) = self.solve_part1() {
            println!("\tPart 1: {}", part_1);
            elapsed_part1 = start.elapsed();
            println!("\tPart 1 took: {:?}\n", elapsed_part1);
        }

        let start = Instant::now();
        let mut elapsed_part2 = Duration::default();
        if let Some(part_2) = self.solve_part2() {
            println!("\tPart 2: {}", part_2);
            elapsed_part2 = start.elapsed();
            println!("\tPart 2 took: {:?}\n", elapsed_part2);
        }

        let total_duration = elapsed_part1 + elapsed_part2;
        println!("\tTotal time: {:?}\n", total_duration);

        total_duration
    }
}
