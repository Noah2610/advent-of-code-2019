//! Advent of Code 2019 - Day 03
//! https://adventofcode.com/2019/day/3

extern crate aoc_util as util;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

type Id = usize;
type Grid = HashMap<Pos, Point>;

fn main() -> Result<(), String> {
    let input = util::get_input();
    let input_lines = input.trim().split("\n");

    let mut circuit = Circuit::default();

    for input_line in input_lines {
        circuit = circuit.with_wire(input_line.to_string())?
    }

    circuit.gen_grid();

    println!("{:#?}", &circuit.grid);

    Ok(())
}

#[derive(PartialEq, Eq, Hash, Default, Clone, Debug)]
struct Pos {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
enum Point {
    Empty,
    Filled(Id),
}

#[derive(Default, Debug)]
struct Circuit {
    grid:  Grid,
    wires: Vec<WirePath>,
}

impl Circuit {
    pub fn with_wire<W>(mut self, try_into_wire: W) -> Result<Self, String>
    where
        W: TryInto<WirePath, Error = String>,
    {
        let wire = try_into_wire.try_into()?;
        self.wires.push(wire);
        Ok(self)
    }

    pub fn gen_grid(&mut self) {
        let mut grid: Grid = HashMap::new();

        for (wire_id, wire) in self.wires.iter().enumerate() {
            let mut pos = Pos::default();

            for direction in wire.0.iter() {
                match direction {
                    Direction::Up(n) => {
                        for y in (1 ..= *n).into_iter() {
                            pos.y -= y as isize;
                            grid.insert(pos.clone(), Point::Filled(wire_id));
                        }
                    }
                    Direction::Down(n) => {
                        for y in (1 ..= *n).into_iter() {
                            pos.y += y as isize;
                            grid.insert(pos.clone(), Point::Filled(wire_id));
                        }
                    }
                    Direction::Left(n) => {
                        for x in (1 ..= *n).into_iter() {
                            pos.x -= x as isize;
                            grid.insert(pos.clone(), Point::Filled(wire_id));
                        }
                    }
                    Direction::Right(n) => {
                        for x in (1 ..= *n).into_iter() {
                            pos.x += x as isize;
                            grid.insert(pos.clone(), Point::Filled(wire_id));
                        }
                    }
                }
            }
        }

        self.grid = grid;
    }
}

#[derive(Debug)]
struct WirePath(pub Vec<Direction>);

impl TryFrom<String> for WirePath {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let directions: Vec<Direction> = s
            .trim()
            .split(",")
            .map(str::trim)
            .try_fold::<_, _, Result<Vec<Direction>, String>>(
            Vec::<Direction>::new(),
            |mut directions, dir_s| {
                directions.push(Direction::try_from(dir_s.to_string())?);
                Ok(directions)
            },
        )?;
        Ok(Self(directions))
    }
}

#[derive(Debug)]
enum Direction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl TryFrom<String> for Direction {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        use std::iter::FromIterator;

        let chars = s.trim().chars().collect::<Vec<char>>();

        if chars.len() >= 2 {
            let n = String::from_iter(&chars[1 ..]).parse::<u32>().or(Err(
                format!(
                    "Couldn't parse numbers string to integer for path \
                     direction: {}",
                    s
                ),
            ))?;

            match chars.get(0).unwrap().to_ascii_uppercase() {
                'U' => Ok(Direction::Up(n)),
                'D' => Ok(Direction::Down(n)),
                'L' => Ok(Direction::Left(n)),
                'R' => Ok(Direction::Right(n)),
                c => Err(format!(
                    "Direction letter must be one of U, D, L, or R. given: {}",
                    c
                )),
            }
        } else {
            Err(format!(
                "Direction string must have at least TWO characters, given: {}",
                s
            ))
        }
    }
}
