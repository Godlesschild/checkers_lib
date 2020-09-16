use crate::{Board, Cell, Piece};
pub struct Checkers {
    board: Board,
    current_white: bool,
}

pub struct Move<'a> {
    pub old_pos: (usize, usize),
    pub new_pos: (usize, usize),
    pub piece: &'a Piece,
    pub captures: Vec<(usize, usize)>,
}

impl<'a> std::fmt::Display for Move<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({};{}) to ({};{})",
            self.old_pos.0 + 1,
            8 - self.old_pos.1,
            self.new_pos.0 + 1,
            8 - self.new_pos.1
        )
    }
}

impl Checkers {
    pub fn new() -> Self {
        Checkers {
            board: Board::new(),
            current_white: true,
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for row in self.board.grid.iter() {
            for cell in row.iter() {
                if let Cell::Piece(piece) = cell {
                    if piece.is_white == self.current_white {
                        moves.append(&mut piece.possible_moves(&self.board))
                    }
                }
            }
        }

        moves
    }
}
