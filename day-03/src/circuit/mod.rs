mod direction;
mod grid;
mod pos;
mod wire;

use direction::Direction;
use grid::Grid;
use pos::Pos;
use std::collections::HashMap;
use std::convert::TryInto;
use wire::Wire;

type Id = char;

#[derive(Default, Debug)]
pub struct Circuit {
    grid:  Grid,
    wires: Vec<Wire>,
}

impl Circuit {
    pub fn with_wire<W>(mut self, try_into_wire: W) -> Result<Self, String>
    where
        W: TryInto<Wire, Error = String>,
    {
        let wire = try_into_wire.try_into()?;
        self.wires.push(wire);
        Ok(self)
    }

    pub fn gen_grid(&mut self) {
        let mut grid = Grid::default();

        for (wire_id, wire) in self.wires.iter().enumerate() {
            let wire_id = wire_id.to_string().chars().next().unwrap();
            grid.add_wire(wire_id, wire);
        }

        self.grid = grid;
    }

    pub fn visualize_grid(&self) -> String {
        format!("{}", &self.grid)
    }

    pub fn closest_manhattan_dist(&self) -> Option<u32> {
        self.grid
            .intersections()
            .iter()
            .map(Pos::calc_manhattan_dist)
            .min()
    }

    pub fn least_steps_to_intersection(&self) -> Option<usize> {
        let mut combined_least_steps = HashMap::<Pos, usize>::new();

        for (wire_id, _) in self.wires.iter().enumerate() {
            let wire_id = wire_id.to_string().chars().next().unwrap();
            let pos = Pos::default();
            let steps_to_each_intersection = self
                .steps_to_intersections_for_wire_at_pos(
                    wire_id,
                    pos,
                    pos,
                    0,
                    &mut Vec::new(),
                );

            let mut intersections_least_steps = HashMap::<Pos, usize>::new();

            steps_to_each_intersection.iter().for_each(|&(pos, steps)| {
                let existing_steps =
                    intersections_least_steps.entry(pos).or_insert(steps);
                if steps < *existing_steps {
                    *existing_steps = steps;
                }
            });

            intersections_least_steps.iter().for_each(|(pos, steps)| {
                let combined_steps =
                    combined_least_steps.entry(*pos).or_insert(0);
                *combined_steps += steps;
            });

            dbg!(&intersections_least_steps);
        }

        let least_steps = combined_least_steps.iter().fold(
            None,
            |mut least_opt, (_, steps)| {
                let least = least_opt.get_or_insert(*steps);
                *least = (*least).min(*steps);
                least_opt
            },
        );

        least_steps
    }

    fn steps_to_intersections_for_wire_at_pos<'a>(
        &self,
        wire_id: Id,
        mut pos: Pos,
        mut prev_pos: Pos,
        orig_steps: usize,
        checked_intersections: &'a mut Vec<Pos>,
    ) -> Vec<(Pos, usize)> {
        let mut steps_to_intersections = Vec::<(Pos, usize)>::new();

        let mut steps = orig_steps;

        loop {
            steps += 1;

            let mut connecting_paths = Vec::new();
            let mut newly_checked_intersections = Vec::new();

            for check_side in
                [Side::Up, Side::Down, Side::Left, Side::Right].iter()
            {
                let check_pos = {
                    let rel = check_side.rel_point();
                    let mut p = pos;
                    p.x += rel[0];
                    p.y += rel[1];
                    p
                };

                if &check_pos != &prev_pos {
                    if let Some(point) = self.grid.point_at(&check_pos) {
                        match point {
                            Point::Start => {
                                connecting_paths.push((check_pos, steps));
                            }
                            Point::Filled(filled_id)
                                if filled_id == &wire_id =>
                            {
                                connecting_paths.push((check_pos, steps));
                            }
                            Point::Intersection(intersections) => {
                                if !checked_intersections.contains(&check_pos) {
                                    if intersections.contains(&wire_id) {
                                        // connecting_paths
                                        //     .push((check_pos, steps));
                                        steps_to_intersections
                                            .push((check_pos, steps));
                                        newly_checked_intersections
                                            .push(check_pos);
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            prev_pos = pos;
            checked_intersections.append(&mut newly_checked_intersections);

            match connecting_paths.len() {
                0 => return steps_to_intersections,
                1 => {
                    let connecting_path = connecting_paths.first().unwrap();
                    pos = connecting_path.0;
                    steps = connecting_path.1;
                }
                n if n > 1 => {
                    for connecting_path in connecting_paths {
                        steps_to_intersections.append(
                            &mut self.steps_to_intersections_for_wire_at_pos(
                                wire_id,
                                connecting_path.0,
                                prev_pos,
                                connecting_path.1,
                                checked_intersections,
                            ),
                        );
                    }
                    return steps_to_intersections;
                }
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug)]
pub enum Point {
    Start,
    Filled(Id),
    Intersection(Vec<Id>),
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Up,
    Down,
    Left,
    Right,
}

impl Side {
    pub fn rel_point(&self) -> [isize; 2] {
        match self {
            Side::Up => [0, -1],
            Side::Down => [0, 1],
            Side::Left => [-1, 0],
            Side::Right => [1, 0],
        }
    }
}
