//! Advent of Code 2019 - Day 01
//! https://adventofcode.com/2019/day/1

extern crate aoc_util as util;

fn main() {
    let input = util::get_input();

    let masses = input
        .split("\n")
        .filter_map(|line_s| line_s.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let fuel_requirements = masses.into_iter().map(calcu_fuel_for_mass);

    let fuel_sum: i32 = fuel_requirements.sum();

    println!("{}", fuel_sum);
}

fn calcu_fuel_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}
