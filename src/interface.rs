use crate::{Board, Move, Piece};
pub struct Checkers {
    board: Board,
    current_white: bool,
}

impl Checkers {
    pub fn new() -> Self {
        Checkers {
            board: Board::new(),
            current_white: true,
        }
    }

    pub fn from_template(template: [[u8; 8]; 8]) -> Self {
        Checkers {
            board: Board::from_template(template),
            current_white: true,
        }
    }

    pub fn draw(&self, draw_numbers: bool) {
        self.board.draw(draw_numbers);
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for row in self.board.grid.iter() {
            for cell in row.iter() {
                if let Some(piece) = cell {
                    if piece.is_white == self.current_white {
                        moves.append(&mut piece.possible_moves(&self.board))
                    }
                }
            }
        }

        moves
    }

    pub fn apply_move(&mut self, apply: Move) {
        self.board.apply_move(apply);
    }

    pub fn applied_move(&self, apply: Move) -> Self {
        Checkers {
            board: self.board.applied_move(apply),
            current_white: !self.current_white,
        }
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
