use core::convert::TryInto;

use itertools::iproduct;

use crate::{Board, Piece, Position, RulesError};

#[derive(Debug, PartialEq, Eq)]
pub struct BoardBuilder {
    board: Board,
    white_pieces: u8,
    black_pieces: u8,
}

impl BoardBuilder {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            board: Board::default(),
            white_pieces: 0,
            black_pieces: 0,
        }
    }

    #[must_use]
    pub fn build(self) -> Board {
        self.board
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// See [`Self::try_insert`].
    #[allow(clippy::missing_panics_doc)]
    pub fn try_from_template(template: [[u8; 8]; 8]) -> Result<Self, crate::Error> {
        let mut builder = Self {
            board: Board::default(),
            white_pieces: 0,
            black_pieces: 0,
        };

        for (x, y) in iproduct!(0..8, 0..8) {
            if template[y][x] != 0 {
                let is_king = template[y][x] > 2;
                let is_white = template[y][x] % 2 == 1;

                builder = builder.try_insert(Piece {
                    is_king,
                    is_white,
                    // never paincs because x and y are both in 0..8
                    position: (x, y).try_into().unwrap(),
                })?;
            }
        }

        Ok(builder)
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// - [`Error::Rules(ColorLimit)`][1], if trying to go over color limit of 12 pieces
    ///   per color,
    /// - [`Error::Rules(Occupied)`][2], if trying to insert a piece into an occupied
    ///   tile,
    /// - [`Error::Rules(NotKing)`][3], if trying to insert a non-king piece into a
    ///   promoting tile.
    ///
    /// [1]: crate::RulesError::ColorLimit
    /// [2]: crate::RulesError::Occupied
    /// [3]: crate::RulesError::NotKing
    pub fn try_insert(mut self, piece: Piece) -> Result<Self, crate::Error> {
        if piece.is_white && self.white_pieces >= 12 {
            Err(RulesError::ColorLimit { is_white: true })?;
        } else if !piece.is_white && self.black_pieces >= 12 {
            Err(RulesError::ColorLimit { is_white: false })?;
        }

        if self.board.get_tile(piece.position).is_some() {
            Err(RulesError::Occupied(piece.position))?;
        }

        if !piece.is_king && piece.position.is_promoting(piece) {
            Err(RulesError::NotKing(piece.position))?;
        }

        self.board.set_tile(piece.position, Some(piece));

        if piece.is_white {
            self.white_pieces += 1;
        } else {
            self.black_pieces += 1;
        }

        Ok(self)
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// - [`Error::Rules(Empty)`][0], if trying to remove a piece from an empty tile.
    ///
    /// [0]: crate::RulesError::Empty
    ///
    pub fn try_remove(mut self, position: Position) -> Result<Self, crate::Error> {
        let Some(Piece {is_white, ..}) = self.board.get_tile(position) else {
            Err(RulesError::Empty(position))?
        };

        self.board.set_tile(position, None);

        if is_white {
            self.white_pieces -= 1;
        } else {
            self.black_pieces -= 1;
        }

        Ok(self)
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// See [`Self::try_insert`], [`Self::try_remove`].
    pub fn try_replace(self, piece: Piece) -> Result<Self, crate::Error> {
        self.try_remove(piece.position)?.try_insert(piece)
    }
}

impl Default for BoardBuilder {
    fn default() -> Self {
        let template: [[u8; 8]; 8] = [
            [0, 2, 0, 2, 0, 2, 0, 2],
            [2, 0, 2, 0, 2, 0, 2, 0],
            [0, 2, 0, 2, 0, 2, 0, 2],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0],
        ];

        BoardBuilder::try_from_template(template).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_empty() -> Result<(), crate::Error> {
        assert_eq!(
            BoardBuilder::empty(),
            BoardBuilder::try_from_template([[0; 8]; 8])?,
        );

        Ok(())
    }

    #[test]
    fn test_from_template() -> Result<(), crate::Error> {
        let template = [
            [0, 2, 0, 0, 0, 0, 0, 3],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 4, 0],
        ];

        let mut result = vec![[
            None,
            Some(Piece {
                is_king: false,
                is_white: false,
                position: 1.try_into()?,
            }),
            None,
            None,
            None,
            None,
            None,
            Some(Piece {
                is_king: true,
                is_white: true,
                position: 4.try_into()?,
            }),
        ]];
        result.extend([[None; 8]; 6]);
        result.push([
            Some(Piece {
                is_king: false,
                is_white: true,
                position: 29.try_into()?,
            }),
            None,
            None,
            None,
            None,
            None,
            Some(Piece {
                is_king: true,
                is_white: false,
                position: 32.try_into()?,
            }),
            None,
        ]);

        assert_eq!(
            BoardBuilder::try_from_template(template)?.build(),
            Board {
                grid: result.try_into().unwrap()
            }
        );

        Ok(())
    }

    #[test]
    fn test_try_insert() -> Result<(), crate::Error> {
        let mut white = Piece {
            is_king: false,
            is_white: true,
            position: 17.try_into()?,
        };

        let mut black_king = Piece {
            is_king: true,
            is_white: false,
            position: 14.try_into()?,
        };

        let board = BoardBuilder::empty()
            .try_insert(white)?
            .try_insert(black_king)?
            .build();

        assert_eq!(board.get_tile(white.position).unwrap(), white);
        assert_eq!(board.get_tile(black_king.position).unwrap(), black_king);

        // Must not go over color limit
        BoardBuilder::default()
            .try_insert(white)
            .expect_err("tried to insert over white color limit");
        BoardBuilder::default()
            .try_insert(black_king)
            .expect_err("tried to insert over black color limit");

        // Must not place into occupied
        white.position = 29.try_into()?;
        black_king.position = 1.try_into()?;
        BoardBuilder::default()
            .try_insert(white)
            .expect_err("tried to insert piece into occupied tile");

        // Must not place non-king piece into promoting tile
        BoardBuilder::empty()
            .try_insert(Piece {
                is_king: false,
                is_white: true,
                position: 1.try_into()?,
            })
            .expect_err("tried to place non-king piece into promoting tile");

        Ok(())
    }

    #[test]
    fn test_try_remove() -> Result<(), crate::Error> {
        BoardBuilder::default()
            .try_remove(1.try_into()?)?
            .try_remove(32.try_into()?)?;

        BoardBuilder::empty()
            .try_remove(1.try_into()?)
            .expect_err("tried to remove from empty tile");

        Ok(())
    }

    #[test]
    fn test_try_replace() -> Result<(), crate::Error> {
        let white = Piece {
            is_king: false,
            is_white: true,
            position: 6.try_into()?,
        };

        let mut black = Piece {
            is_king: false,
            is_white: false,
            position: 6.try_into()?,
        };

        assert_eq!(
            BoardBuilder::empty()
                .try_insert(white)?
                .try_replace(black)?,
            BoardBuilder::try_from_template([
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 2, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0]
            ])?
        );

        // Must not replace in empty tile
        BoardBuilder::empty()
            .try_replace(white)
            .expect_err("tried to replace empty tile");

        // Must not go over color limit
        black.position = 32.try_into()?;
        BoardBuilder::default()
            .try_replace(white)
            .expect_err("tried to replace over white color limit");
        BoardBuilder::default()
            .try_replace(black)
            .expect_err("tried to replace over black color limit");

        Ok(())
    }
}
