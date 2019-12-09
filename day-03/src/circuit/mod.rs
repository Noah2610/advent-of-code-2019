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
type IntersectionId = String;

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

    pub fn least_steps_to_intersection(&self) -> Option<u32> {
        for (wire_id, _) in self.wires.iter().enumerate() {
            let wire_id = wire_id.to_string().chars().next().unwrap();
            let pos = Pos::default();
            let steps_to_each_intersection = self
                .steps_to_intersections_for_wire_at_pos(
                    wire_id,
                    pos,
                    vec![pos],
                    0,
                );
            dbg!(&wire_id);
            dbg!(&steps_to_each_intersection);
        }

        None
    }

    fn steps_to_intersections_for_wire_at_pos(
        &self,
        wire_id: Id,
        orig_pos: Pos,
        mut checked_poss: Vec<Pos>,
        orig_steps: usize,
    ) -> Vec<(IntersectionId, usize)> {
        let mut steps_to_intersections = Vec::<(IntersectionId, usize)>::new();
        let mut pos = orig_pos;

        // TODO
        let mut steps = orig_steps;

        loop {
            steps += 1;

            let mut connecting_paths = Vec::new();

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

                if !checked_poss.contains(&check_pos) {
                    if let Some(point) = self.grid.point_at(&check_pos) {
                        match point {
                            Point::Start => {
                                // TODO
                                connecting_paths.push(check_pos);
                            }
                            Point::Filled(filled_id)
                                if filled_id == &wire_id =>
                            {
                                // steps += 1;
                                connecting_paths.push(check_pos);

                                // steps_to_intersections.append(
                                //     &mut self
                                //         .steps_to_intersections_for_wire_at_pos(
                                //             wire_id, check_pos, pos,
                                //         ),
                                // );
                            }
                            Point::Intersection(intersections) => {
                                // steps += 1;
                                connecting_paths.push(check_pos);
                                let intersection_id =
                                    id_for_intersection(&intersections);
                                steps_to_intersections
                                    .push((intersection_id, steps));
                            }
                            _ => (),
                        }
                    }
                }
            }

            match connecting_paths.len() {
                0 => return steps_to_intersections,
                1 => {
                    checked_poss.push(pos);
                    // prev_pos = pos;
                    pos = connecting_paths.first().unwrap().clone();
                }
                n if n > 1 => {
                    for connecting_path in connecting_paths {
                        steps_to_intersections.append(
                            &mut self.steps_to_intersections_for_wire_at_pos(
                                wire_id,
                                connecting_path,
                                checked_poss.clone(),
                                steps,
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

fn id_for_intersection(intersections: &Vec<Id>) -> IntersectionId {
    intersections.iter().collect::<String>()
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
