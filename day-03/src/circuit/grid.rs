use super::{Direction, Id, Point, Pos, Wire};
use crate::grid_chars;
use std::collections::HashMap;
use std::fmt;

#[derive(Default, Debug)]
pub struct Grid {
    points:        HashMap<Pos, Point>,
    intersections: Vec<Pos>,
}

impl Grid {
    pub fn add_wire(&mut self, wire_id: Id, wire: &Wire) {
        let mut pos = Pos::default();

        self.insert(pos.clone(), Point::Start);

        for direction in wire.0.iter() {
            match direction {
                Direction::Up(n) => {
                    for _ in (0 .. *n).into_iter() {
                        pos.y -= 1 as isize;
                        self.add(pos.clone(), wire_id);
                    }
                }
                Direction::Down(n) => {
                    for _ in (0 .. *n).into_iter() {
                        pos.y += 1 as isize;
                        self.add(pos.clone(), wire_id);
                    }
                }
                Direction::Left(n) => {
                    for _ in (0 .. *n).into_iter() {
                        pos.x -= 1 as isize;
                        self.add(pos.clone(), wire_id);
                    }
                }
                Direction::Right(n) => {
                    for _ in (0 .. *n).into_iter() {
                        pos.x += 1 as isize;
                        self.add(pos.clone(), wire_id);
                    }
                }
            }
        }
    }

    pub fn point_at(&self, pos: &Pos) -> Option<&Point> {
        self.points.get(pos)
    }

    pub fn intersections(&self) -> Vec<Pos> {
        self.intersections.clone()
    }

    fn insert(&mut self, pos: Pos, point: Point) {
        self.points.insert(pos, point);
    }

    fn add(&mut self, pos: Pos, id: Id) {
        if let Some(existing_point) = self.points.get_mut(&pos) {
            match existing_point {
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
                if let Some(point) = self.point_at(&pos) {
                    string.push(match point {
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
