mod direction;
mod grid;
mod pos;
mod wire;

use direction::Direction;
use grid::Grid;
use pos::Pos;
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
            grid.add_wire(wire_id.to_string().chars().next().unwrap(), wire);
        }

        self.grid = grid;
    }

    pub fn closest_manhattan_dist(&self) -> Option<u32> {
        self.grid
            .intersections()
            .iter()
            .map(Pos::calc_manhattan_dist)
            .min()
    }
}

#[derive(Debug)]
pub enum Point {
    Start,
    Filled(Id),
    Intersection(Vec<Id>),
}
