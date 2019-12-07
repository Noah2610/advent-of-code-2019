//! Advent of Code 2019 - Day 01
//! https://adventofcode.com/2019/day/1

extern crate aoc_util as util;

fn main() {
    println!("{}", calc_fuel());
}

fn calc_fuel() -> i32 {
    let input = util::get_input();
    input
        .split("\n")
        .filter_map(|line_s| line_s.parse::<i32>().ok())
        .map(calc_fuel_for_mass)
        .sum()
}

fn calc_fuel_for_mass(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + calc_fuel_for_mass(fuel)
    } else {
        0
    }
}

#[test]
fn test_correct_answer() {
    assert_eq!(5182078, calc_fuel());
}
