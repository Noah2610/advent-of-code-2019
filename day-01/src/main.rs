//! Advent of Code 2019 - Day 01
//! https://adventofcode.com/2019/day/1

extern crate aoc_util as util;

fn main() {
    let input = util::get_input();

    let fuel_sum: i32 = input
        .split("\n")
        .filter_map(|line_s| line_s.parse::<i32>().ok())
        .map(calc_fuel_for_mass)
        .sum();

    println!("{}", fuel_sum);
}

fn calc_fuel_for_mass(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + calc_fuel_for_mass(fuel)
    } else {
        0
    }
}
