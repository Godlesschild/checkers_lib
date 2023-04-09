use core::fmt::Write;

use crate::{CheckersMove, Piece, Position, RulesError};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Board {
    pub(crate) grid: [[Option<Piece>; 8]; 8],
}

impl Board {
    #[must_use]
    pub fn all_possible_moves(&self, current_white: bool) -> Vec<CheckersMove> {
        let mut moves = Vec::new();

        for piece in self.iter().flatten().flatten() {
            if piece.is_white == current_white {
                moves.append(&mut piece.all_possible_moves(self));
            }
        }

        if moves.iter().any(|i| !i.captures.is_empty()) {
            moves.retain(|i| !i.captures.is_empty());
        }

        // let mut can_capture = false;
        // for piece in self.iter().flatten().flatten() {
        //     if piece.is_white == current_white {
        //         let possible = &mut piece.all_possible_moves(self);

        //         if !can_capture && possible.iter().any(f) {
        //             moves.retain(f);
        //             can_capture = true;
        //         }

        //         if can_capture {
        //             possible.retain(f);
        //         }
        //         moves.append(possible);
        //     }
        // }

        moves
    }

    #[must_use]
    pub fn legal_moves(&self, current_white: bool) -> Vec<CheckersMove> {
        let possible_moves = self.all_possible_moves(current_white);
        let mut legal_moves = Vec::new();

        let captured_any = possible_moves.iter().any(|i| !i.captures.is_empty());

        if captured_any {
            for i in possible_moves {
                if !i.captures.is_empty() {
                    legal_moves.push(i);
                }
            }

            legal_moves
        } else {
            possible_moves
        }
    }

    /// Applies a [`CheckersMove`] to `self` in-place.
    ///
    /// This function will either return an [`Error::Rules`][1] or result in a jumbled up board
    /// if passed a move that does not correspond with `self` (e.g. generated from a
    /// different board).
    ///
    /// # Errors
    ///
    /// This function returns an [`Error::Rules`][1] if it can't apply the move. Even though
    /// this function will most likely fail in case the move does not correspond with
    /// `self` (e.g. generated from a different board), it still can not guarantee to
    /// result in a valid board, so it's up to the user to make sure that moves are
    /// only applied to the correct board.
    ///
    /// [1]: crate::RulesError
    pub fn apply_move(&mut self, apply: &CheckersMove) -> Result<(), RulesError> {
        if self.get_tile(apply.old_piece().position).is_none() {
            Err(RulesError::Empty(apply.old_piece().position))?;
        };

        self.set_tile(apply.new_piece().position, Some(apply.new_piece()));

        self.set_tile(apply.old_piece().position, None);

        for position in &apply.captures {
            if self.get_tile(*position).is_none() {
                Err(RulesError::Empty(*position))?;
            }
            self.set_tile(*position, None);
        }

        Ok(())
    }

    /// Applies a [`CheckersMove`] to `self` in-place.
    ///
    /// This function will either panic or result in a jumbled up board if passed a move
    /// that does not correspond with `self` (e.g. generated from a different board).
    ///
    /// # Panics
    ///
    /// - tile at position of moving piece is empty
    ///
    pub fn apply_move_unchecked(&mut self, apply: &CheckersMove) {
        self.set_tile(apply.new_piece().position, Some(apply.new_piece()));

        self.set_tile(apply.old_piece().position, None);

        for position in &apply.captures {
            self.set_tile(*position, None);
        }
    }

    /// Consuming variant of [`Self::apply_move`]
    ///
    /// # Errors
    ///
    /// See [`Self::apply_move`]
    ///
    pub fn applied_move(mut self, apply: &CheckersMove) -> Result<Self, RulesError> {
        self.apply_move(apply)?;
        Ok(self)
    }

    /// Consuming variant of [`Self::apply_move_unchecked`]
    ///
    /// # Panics
    ///
    /// See [`Self::apply_move_unchecked`]
    ///
    #[must_use]
    pub fn applied_move_unchecked(mut self, apply: &CheckersMove) -> Self {
        self.apply_move_unchecked(apply);
        self
    }

    pub fn iter(&self) -> std::slice::Iter<'_, [Option<Piece>; 8]> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, [Option<Piece>; 8]> {
        self.into_iter()
    }

    #[must_use]
    pub fn get_tile(&self, pos: Position) -> Option<Piece> {
        let (x, y) = pos.into();

        self.grid[y][x]
    }

    /// # Safety
    ///
    /// The caller must ensure that the tile at `pos` is not empty.
    ///
    /// Use with `pos` pointing to an empty tile is UB.
    ///
    #[must_use]
    pub unsafe fn get_tile_unchecked(&self, pos: Position) -> Piece {
        let (x, y) = pos.into();

        self.grid[y][x].unwrap_unchecked()
    }

    pub(crate) fn set_tile(&mut self, pos: Position, piece: Option<Piece>) {
        let (x, y) = pos.into();

        self.grid[y][x] = piece;
    }
}

impl IntoIterator for Board {
    type Item = [Option<Piece>; 8];

    type IntoIter = std::array::IntoIter<[Option<Piece>; 8], 8>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.grid)
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a [Option<Piece>; 8];

    type IntoIter = std::slice::Iter<'a, [Option<Piece>; 8]>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.iter()
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut [Option<Piece>; 8];

    type IntoIter = std::slice::IterMut<'a, [Option<Piece>; 8]>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.iter_mut()
    }
}

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();

        for (num, row) in self.iter().enumerate() {
            write!(buf, "{} ", self.grid.len() - num)?;

            for tile in row.iter() {
                write!(
                    buf,
                    "{} ",
                    match tile {
                        None => "_".to_owned(),
                        Some(piece) => format!("{piece}"),
                    }
                )?;
            }
            writeln!(buf)?;
        }

        write!(buf, "  ")?;
        for num in 1..=self.grid.len() {
            write!(buf, "{num} ")?;
        }
        writeln!(buf)?;

        write!(f, "{buf}")
    }
}

impl core::fmt::Debug for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Display::fmt(&self, f)
    }
}
