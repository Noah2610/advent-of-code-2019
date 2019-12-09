//! Advent of Code 2019 - Day 03
//! https://adventofcode.com/2019/day/3

extern crate aoc_util as util;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;

type Id = char;

mod grid_chars {
    pub const EMPTY: char = ' ';
    pub const START: char = '#';
    pub const INTERSECTION: char = 'X';
}

fn main() -> Result<(), String> {
    let input = util::get_input();
    let input_lines = input.trim().split("\n");

    let mut circuit = Circuit::default();

    for input_line in input_lines {
        circuit = circuit.with_wire(input_line.to_string())?
    }

    circuit.gen_grid();

    println!("{}\n\n{:#?}", &circuit.grid, circuit.grid.intersections());

    Ok(())
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
        let mut grid = Grid::default();

        for (wire_id, wire) in self.wires.iter().enumerate() {
            let mut pos = Pos::default();
            let wire_id = wire_id.to_string().chars().next().unwrap();

            grid.insert(pos.clone(), Point::Start);

            for direction in wire.0.iter() {
                match direction {
                    Direction::Up(n) => {
                        for _ in (0 .. *n).into_iter() {
                            pos.y -= 1 as isize;
                            grid.add(pos.clone(), wire_id);
                        }
                    }
                    Direction::Down(n) => {
                        for _ in (0 .. *n).into_iter() {
                            pos.y += 1 as isize;
                            grid.add(pos.clone(), wire_id);
                        }
                    }
                    Direction::Left(n) => {
                        for _ in (0 .. *n).into_iter() {
                            pos.x -= 1 as isize;
                            grid.add(pos.clone(), wire_id);
                        }
                    }
                    Direction::Right(n) => {
                        for _ in (0 .. *n).into_iter() {
                            pos.x += 1 as isize;
                            grid.add(pos.clone(), wire_id);
                        }
                    }
                }
            }
        }

        self.grid = grid;
    }
}

#[derive(Default, Debug)]
struct Grid {
    points:        HashMap<Pos, Point>,
    intersections: Vec<Pos>,
}

impl Grid {
    pub fn insert(&mut self, pos: Pos, point: Point) {
        self.points.insert(pos, point);
    }

    pub fn get(&self, pos: &Pos) -> Option<&Point> {
        self.points.get(pos)
    }

    pub fn add(&mut self, pos: Pos, id: Id) {
        if let Some(existing_point) = self.points.get_mut(&pos) {
            match existing_point {
                Point::Empty => {
                    *existing_point = Point::Filled(id);
                }
                Point::Start => {
                    // Don't overwrite starting point, but also don't see it as an intersection.
                    // Not sure if this is the correct behaviour.
                }
                Point::Filled(existing_id) => {
                    if *existing_id != id {
                        *existing_point =
                            Point::Intersection(vec![*existing_id, id]);
                        self.intersections.push(pos);
                    }
                }
                Point::Intersection(existing_ids) => {
                    if !existing_ids.contains(&id) {
                        existing_ids.push(id);
                        self.intersections.push(pos);
                    }
                }
            }
        } else {
            self.insert(pos, Point::Filled(id));
        }
    }

    pub fn intersections(&self) -> &Vec<Pos> {
        &self.intersections
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut y_bounds = (0, 0);
        let mut x_bounds = (0, 0);

        for pos in self.points.keys() {
            if pos.y < y_bounds.0 {
                y_bounds.0 = pos.y;
            } else if pos.y > y_bounds.1 {
                y_bounds.1 = pos.y;
            }
            if pos.x < x_bounds.0 {
                x_bounds.0 = pos.x;
            } else if pos.x > x_bounds.1 {
                x_bounds.1 = pos.x;
            }
        }

        let mut lines = Vec::new();

        for y in y_bounds.0 ..= y_bounds.1 {
            let mut string = String::new();

            for x in x_bounds.0 ..= x_bounds.1 {
                let pos = Pos::new(x, y);
                if let Some(point) = self.get(&pos) {
                    string.push(match point {
                        Point::Empty => grid_chars::EMPTY,
                        Point::Start => grid_chars::START,
                        Point::Filled(id) => *id,
                        Point::Intersection(_) => grid_chars::INTERSECTION,
                    });
                } else {
                    string.push(grid_chars::EMPTY);
                }
            }

            lines.push(string);
        }

        write!(f, "{}", lines.join("\n"))
    }
}

#[derive(PartialEq, Eq, Hash, Default, Clone, Debug)]
struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
enum Point {
    Empty,
    Start,
    Filled(Id),
    Intersection(Vec<Id>),
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
