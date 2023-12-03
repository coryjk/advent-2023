mod problem;
mod days;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::problem::Problem;
use crate::days::day01::Day1;

fn main() {
    let day1 = Day1 { input: load_input("./inputs/day01.txt") };
    println!("day 01, part 01: {}", day1.solve_part_one());
    println!("day 01, part 02: {}", day1.solve_part_two());
}

fn load_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}