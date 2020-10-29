use std::convert::TryFrom;

#[derive(Clone, Copy)]
pub struct Position(PositionInner);

#[derive(Clone, Copy)]
enum PositionInner {
    Coordinates(usize, usize),
    Notation(u8),
}

impl Position {
    pub fn from_notation(num: u8) -> Result<Self, &'static str> {
        if num > 32 {
            Err("illegal position, notation number too large")
        } else if num == 0 {
            Err("illegal position, notation number is 0")
        } else {
            Ok(Position(PositionInner::Notation(num)))
        }
    }

    pub fn from_coordinates((x, y): (usize, usize)) -> Result<Self, &'static str> {
        if x > 7 || y > 7 {
            Err("illegal position, coordinates too large")
        } else if (x + y) % 2 == 0 {
            Err("illegal position, piece on white square")
        } else {
            Ok(Position(PositionInner::Coordinates(x, y)))
        }
    }

    pub fn as_coordinates(&self) -> (usize, usize) {
        match self {
            Position(PositionInner::Coordinates(x, y)) => (*x, *y),
            Position(PositionInner::Notation(num)) => {
                let y = (num - 1) / 4;
                let x = (num - 1) % 4 * 2 + (y + 1) % 2;

                (usize::from(x), usize::from(y))
            }
        }
    }

    pub fn as_notation(&self) -> u8 {
        match self {
            Position(PositionInner::Notation(num)) => *num,
            Position(PositionInner::Coordinates(x, y)) => u8::try_from(y * 4 + x / 2 + 1).unwrap(),
        }
    }
}
