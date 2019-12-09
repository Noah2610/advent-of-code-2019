use std::convert::TryFrom;

#[derive(Debug)]
pub enum Direction {
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
