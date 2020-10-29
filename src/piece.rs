use crate::{Board, CheckersMove, Position};

#[derive(Copy, Clone)]
pub struct Piece {
    pub is_king: bool,
    pub is_white: bool,
    pub position: Position,
}

impl Piece {
    fn next_y(piece: &Self, y: Option<usize>) -> usize {
        let current_y = if let Some(y) = y {
            y
        } else {
            piece.position.as_coordinates().1
        };
        if piece.is_white == piece.is_king {
            current_y + 1
        } else {
            current_y - 1
        }
    }

    fn check_capture<'a>(
        piece: &'a Self,
        board: &Board,
        capture_pos: (usize, usize),
        new_pos: (usize, usize),
    ) -> Option<CheckersMove<'a>> {
        if let (Some(other), None) = (
            board.grid[capture_pos.1][capture_pos.0],
            board.grid[new_pos.1][new_pos.0],
        ) {
            if piece.is_white != other.is_white {
                return Some(CheckersMove {
                    old_pos: piece.position,
                    new_pos: Position::from_coordinates(new_pos).unwrap(),
                    piece,
                    captures: vec![Position::from_coordinates(capture_pos).unwrap()],
                });
            }
        }
        return None;
    }

    fn possible_captures(&self, board: &Board) -> Vec<CheckersMove> {
        let mut captures = Vec::new();

        if !self.is_king {
            let capture_y = Self::next_y(&self, None);
            if capture_y >= 7 || capture_y <= 0 {
                return Vec::new();
            };

            let self_x = self.position.as_coordinates().0;

            // To the left
            if self_x > 1 {
                let capture_pos = (self_x - 1, capture_y);
                let new_pos = (capture_pos.0 - 1, Self::next_y(&self, Some(capture_pos.1)));

                if let Some(capture) = Self::check_capture(&self, board, capture_pos, new_pos) {
                    captures.push(capture);
                }
            }

            // To the right
            if self_x < 6 {
                let capture_pos = (self_x + 1, capture_y);
                let new_pos = (capture_pos.0 + 1, Self::next_y(&self, Some(capture_pos.1)));

                if let Some(capture) = Self::check_capture(&self, board, capture_pos, new_pos) {
                    captures.push(capture);
                }
            }

            return captures;
        } else {
            return Vec::new(); // ! TEMPORARY
                               // TODO Add kings
        }
    }

    pub fn possible_moves(&self, board: &Board) -> Vec<CheckersMove> {
        let mut moves = self.possible_captures(board);

        if !moves.is_empty() {
            for (i, capture) in moves.clone().iter().enumerate() {
                let (x, y) = capture.new_pos.as_coordinates();

                let board_next = board.applied_move(capture);
                let piece_next = board_next.grid[y][x].unwrap();

                let mut recursive_moves = piece_next.possible_moves(&board_next);

                let mut captured_any = false;
                for recursive_move in recursive_moves.iter() {
                    if !recursive_move.captures.is_empty() {
                        captured_any = true;
                    }
                }

                let CheckersMove { captures, .. } = moves.remove(i);

                if captured_any {
                    for recursive_move in recursive_moves.iter_mut() {
                        let mut captures = captures.clone();
                        captures.append(&mut recursive_move.captures);

                        moves.push(CheckersMove {
                            old_pos: self.position,
                            new_pos: recursive_move.new_pos,
                            piece: &self,
                            captures,
                        });
                    }
                } else {
                    moves.push(CheckersMove {
                        old_pos: self.position,
                        new_pos: piece_next.position,
                        piece: &self,
                        captures: captures,
                    })
                }
            }
        } else {
            let next_y = Self::next_y(&self, None);
            let self_x = self.position.as_coordinates().0;

            // To the left
            if self_x > 0 {
                let new_pos = (self_x - 1, next_y);

                if board.grid[new_pos.1][new_pos.0].is_none() {
                    moves.push(CheckersMove {
                        old_pos: self.position,
                        new_pos: Position::from_coordinates(new_pos).unwrap(),
                        piece: &self,
                        captures: Vec::new(),
                    });
                }
            }

            // To the right
            if self_x < 7 {
                let new_pos = (self_x + 1, next_y);

                if board.grid[new_pos.1][new_pos.0].is_none() {
                    moves.push(CheckersMove {
                        old_pos: self.position,
                        new_pos: Position::from_coordinates(new_pos).unwrap(),
                        piece: &self,
                        captures: Vec::new(),
                    });
                }
            }
        }

        moves
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
