use std::collections::HashSet;

use itertools::iproduct;

use crate::{Board, CheckersMove, Position, RulesError};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub is_king: bool,
    pub is_white: bool,
    pub position: Position,
}

impl Piece {
    #[must_use]
    pub fn new(is_king: bool, is_white: bool, position: Position) -> Self {
        Self {
            is_king,
            is_white,
            position,
        }
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// - [`Error::Rules(Occupied)`][0] if another piece is obsturcting movement,
    /// - [`Error::Rules(NotKing)`][1] if trying to move backwars without being a king,
    /// - [`Error::Position(OutOfBounds)`][2] if `direction` points immediatly out of
    ///   bounds.
    ///
    /// [0]: crate::RulesError::Occupied
    /// [1]: crate::RulesError::NotKing
    /// [2]: crate::position::Error::OutOfBounds
    pub fn moves(
        self,
        board: &Board,
        direction: (i8, i8),
    ) -> Result<Vec<CheckersMove>, crate::Error> {
        if !self.is_king
            && ((direction.1 > 0) && self.is_white || ((direction.1 < 0) && !self.is_white))
        {
            Err(RulesError::NotKing(self.position))?;
        }

        let mut new_pos = self.position.increment(direction)?;

        if board.get_tile(new_pos).is_some() {
            Err(RulesError::Occupied(new_pos))?;
        }

        let mut moves = vec![CheckersMove {
            old: self,
            new: Piece {
                is_king: self.is_king || new_pos.is_promoting(self),
                is_white: self.is_white,
                position: new_pos,
            },
            captures: HashSet::new(),
        }];

        if self.is_king {
            while let Ok(temp) = new_pos.increment(direction) {
                new_pos = temp;

                if board.get_tile(new_pos).is_some() {
                    break;
                }

                let mut next_move = moves[0].clone();
                next_move.new.position = new_pos;

                moves.push(next_move);
            }
        }

        Ok(moves)
    }

    /// TODO
    ///
    /// # Errors
    ///
    /// - [`Error::Rules(SameColorCapture)`][0] if trying to capture piece of same color,
    /// - [`Error::Rules(Empty)`][1] if there is nothing to capture in given `direction`,
    /// - [`Error::Rules(Occupied)`][2] if the tile after capture is occupied,
    /// - [`Error::Position(OutOfBounds)`][3] if `direction` points immediatly out of bounds or
    ///   there is no tile to land on after capture.
    ///
    /// [0]: crate::RulesError::SameColorCapture
    /// [1]: crate::RulesError::Empty
    /// [2]: crate::RulesError::Occupied
    /// [3]: crate::position::Error::OutOfBounds
    pub fn capture_in_direction(
        self,
        board: &Board,
        direction: (i8, i8),
    ) -> Result<Vec<CheckersMove>, crate::Error> {
        let mut capture_pos = self.position.increment(direction)?;

        if self.is_king {
            while board.get_tile(capture_pos).is_none() {
                let Ok(next_capture_pos) = capture_pos.increment(direction) else {
                    Err(RulesError::Empty(capture_pos))?
                };

                capture_pos = next_capture_pos;
            }
        }

        let mut new_pos = capture_pos.increment(direction)?;

        match (board.get_tile(capture_pos), board.get_tile(new_pos)) {
            (None, _) => Err(RulesError::Empty(capture_pos))?,
            (_, Some(_)) => Err(RulesError::Occupied(new_pos))?,

            (Some(other), None) => {
                if self.is_white == other.is_white {
                    Err(RulesError::SameColorCapture {
                        capturing_pos: self.position,
                        captured_pos: other.position,
                        is_white: self.is_white,
                    })?;
                }
            }
        }

        let mut captures = Vec::new();

        captures.push(CheckersMove {
            old: self,
            new: Piece {
                is_king: self.is_king || new_pos.is_promoting(self),
                is_white: self.is_white,
                position: new_pos,
            },
            captures: HashSet::from([capture_pos]),
        });

        if self.is_king {
            while let Ok(temp) = new_pos.increment(direction) {
                new_pos = temp;
                if board.get_tile(new_pos).is_some() {
                    break;
                }

                let mut next_move = captures[0].clone();
                next_move.new.position = new_pos;

                captures.push(next_move);
            }
        }

        Ok(captures)
    }

    #[must_use]
    fn capture_in_all_directions(self, board: &Board) -> Vec<CheckersMove> {
        let mut all_captures = Vec::new();

        for direction in iproduct!([-1, 1], [-1, 1]) {
            if let Ok(mut captures) = self.capture_in_direction(board, direction) {
                all_captures.append(&mut captures);
            }
        }

        all_captures
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn all_possible_moves(self, board: &Board) -> Vec<CheckersMove> {
        let possible_captures = self.capture_in_all_directions(board);
        let mut all_possible_moves = Vec::new();

        // if piece can't capture
        if possible_captures.is_empty() {
            for direction in iproduct!([-1, 1], [-1, 1]) {
                if let Ok(mut moves) = self.moves(board, direction) {
                    all_possible_moves.append(&mut moves);
                }
            }

            return all_possible_moves;
        }

        // if piece can capture
        for current_capture in possible_captures {
            let board_next = board.clone().applied_move_unchecked(&current_capture);
            let piece_next =
                unsafe { board_next.get_tile_unchecked(current_capture.new_piece().position) };
            // SAFETY: ↑ tile at `capture.new.position` in `board_next` is presumed to never be empty

            let mut recursive_captures = piece_next
                .all_possible_moves(&board_next)
                .into_iter()
                .filter(|i| !i.captures.is_empty())
                .collect::<Vec<_>>();

            for recursive_capture in &mut recursive_captures {
                recursive_capture.old = current_capture.old_piece();
                recursive_capture
                    .captures
                    .extend(current_capture.captures.clone());
            }

            if recursive_captures.is_empty() {
                recursive_captures.push(current_capture);
            }

            all_possible_moves.append(&mut recursive_captures);
        }

        all_possible_moves
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // White pieces
                Piece {
                    is_king: false,
                    is_white: true,
                    ..
                } => "●",
                Piece {
                    is_king: true,
                    is_white: true,
                    ..
                } => "◉",

                // Black pieces
                Piece {
                    is_king: false,
                    is_white: false,
                    ..
                } => "◯",
                Piece {
                    is_king: true,
                    is_white: false,
                    ..
                } => "◎",
            }
        )
    }
}
