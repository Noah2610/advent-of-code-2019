//! Advent of Code 2019 - Day 03
//! https://adventofcode.com/2019/day/3

extern crate aoc_util as util;

mod circuit;
mod grid_chars;

use circuit::Circuit;

fn main() -> Result<(), String> {
    let input = util::get_input();
    let input_lines = input.trim().split("\n");

    let mut circuit = Circuit::default();

    for input_line in input_lines {
        circuit = circuit.with_wire(input_line.to_string())?
    }

    circuit.gen_grid();

    if let Some(dist) = circuit.closest_manhattan_dist() {
        println!("Closest Manhattan Distance:\n{}", dist);
    } else {
        println!("No intersections found!");
    }

    Ok(())
}
