// use std::convert::TryFrom;

use crate::Piece;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("position out of bounds (must be in 1..=32 or (0..8, 0..8))")]
    OutOfBounds,

    #[error("position points to white square")]
    WhiteSquare,
}

#[derive(Clone, Copy)]
pub struct Position(u8);

impl Position {
    /// Return `self` with `x` and `y` incremented by `delta.0` and `delta.1`
    /// respectively.
    ///
    /// # Errors
    ///
    /// - [`Error::Position(OutOfBounds)`][0] if position goes out of bounds after
    ///   increment.
    ///
    /// [0]: Error::OutOfBounds
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    pub fn increment(self, (dx, dy): (i8, i8)) -> Result<Self, Error> {
        let (x, y): (usize, usize) = self.into();
        let (x, y): (i8, i8) = (x as i8 + dx, y as i8 + dy);

        match (x, y) {
            (0..=7, 0..=7) => Ok((x as usize, y as usize).try_into()?),
            _ => Err(Error::OutOfBounds),
        }
    }

    /// Returns if a piece on this tile should be promoted.
    #[must_use]
    pub fn is_promoting(self, piece: Piece) -> bool {
        matches!((piece.is_white, self.0), (true, 1..=4) | (false, 29..=32))
    }
}

/// Tries to convert to a `Position` from a `u8` square number (draughts notation).
impl TryFrom<u8> for Position {
    type Error = Error;

    fn try_from(num: u8) -> Result<Self, Self::Error> {
        match num {
            0 | 33.. => Err(Error::OutOfBounds),
            _ => Ok(Position(num)),
        }
    }
}

/// Tries to convert to a `Position` from a coordinate tuple.
impl TryFrom<(usize, usize)> for Position {
    type Error = Error;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(coords: (usize, usize)) -> Result<Self, Self::Error> {
        match coords {
            (8.., 8..) => Err(Error::OutOfBounds),
            (x, y) => {
                if (x + y) % 2 == 0 {
                    Err(Error::WhiteSquare)
                } else {
                    Ok(Position((y as u8) * 4 + (x as u8) / 2 + 1))
                }
            }
        }
    }
}

/// Converts a Position to a u8 square number (draughts notation).
impl From<Position> for u8 {
    fn from(Position(num): Position) -> Self {
        num
    }
}

impl From<Position> for (usize, usize) {
    fn from(Position(num): Position) -> Self {
        let y = (num - 1) / 4;
        let x = (num - 1) % 4 * 2 + (y + 1) % 2;

        (usize::from(x), usize::from(y))
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        let s: u8 = (*self).into();
        let o: u8 = (*other).into();

        s == o
    }
}

impl Eq for Position {}

impl std::hash::Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let coords: (usize, usize) = (*self).into();
        write!(f, "{coords:?}")
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    use itertools::iproduct;
    use std::convert::TryInto;

    #[test]
    fn test_from_coords() -> Result<(), Error> {
        for coords in iproduct!(0..8usize, 0..8usize).filter(|(x, y)| (x + y) % 2 != 0) {
            TryInto::<Position>::try_into(coords)?;
        }

        for coords in iproduct!(8..15usize, 8..15usize) {
            TryInto::<Position>::try_into(coords).expect_err("creates out of bounds position");
        }

        Ok(())
    }

    #[test]
    fn test_from_notation() -> Result<(), Error> {
        TryInto::<Position>::try_into(0u8).expect_err("creates position from zero");

        for i in 1..=32u8 {
            let _: Position = i.try_into()?;
        }

        for i in 33..40u8 {
            TryInto::<Position>::try_into(i).expect_err("creates out of bounds position");
        }

        Ok(())
    }

    #[test]
    fn test_to_coords() -> Result<(), Error> {
        let pos: Position = 6u8.try_into()?;
        let coords: (usize, usize) = pos.into();

        assert_eq!(coords, (2, 1));

        Ok(())
    }

    #[test]
    fn test_to_notation() -> Result<(), Error> {
        let pos: Position = (2, 1).try_into()?;
        let not: u8 = pos.into();

        assert_eq!(not, 6);

        Ok(())
    }

    #[test]
    fn test_eq() -> Result<(), crate::Error> {
        let a: Position = 1.try_into()?;
        let b: Position = 2.try_into()?;
        let c: Position = 1.try_into()?;

        assert_ne!(a, b);
        assert_eq!(a, c);

        Ok(())
    }
}
