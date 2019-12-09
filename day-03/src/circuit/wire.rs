use super::Direction;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Wire(pub Vec<Direction>);

impl TryFrom<String> for Wire {
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
