//! Advent of Code 2019 - Day 01
//! https://adventofcode.com/2019/day/1

extern crate aoc_util as util;

fn main() {
    let input = util::get_input();

    let masses = input
        .split("\n")
        .filter_map(|line_s| line_s.parse::<i32>().ok())
        .collect();

    let fuel = calc_fuel_requirement(masses);

    println!("{}", fuel);
}

fn calc_fuel_requirement(masses: Vec<i32>) -> i32 {
    let fuel_requirements = masses
        .into_iter()
        .map(calc_fuel_for_mass)
        .collect::<Vec<i32>>();

    fuel_requirements.iter().sum::<i32>()
}

fn calc_fuel_for_mass(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + calc_fuel_for_mass(fuel)
    } else {
        0
    }
}
