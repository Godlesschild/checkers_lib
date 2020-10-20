pub mod interface;
pub use interface::Checkers;

#[derive(Clone)]
pub struct Board {
    grid: [[Option<Piece>; 8]; 8],
}

#[derive(Copy, Clone)]
pub struct Piece {
    is_king: bool,
    is_white: bool,
    position: (usize, usize),
}

#[derive(Clone)]
pub struct Move<'a> {
    pub old_pos: (usize, usize),
    pub new_pos: (usize, usize),
    pub piece: &'a Piece,
    pub captures: Vec<(usize, usize)>,
}

impl Board {
    fn new() -> Self {
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

        Self::from_template(template)
    }

    fn from_template(template: [[u8; 8]; 8]) -> Self {
        let mut board = Board {
            grid: [[None; 8]; 8],
        };

        for y in 0..8usize {
            for x in 0..8usize {
                let cell = template[y][x];
                if cell != 0 {
                    board.grid[y][x] = Some(Piece {
                        is_king: cell > 2,
                        is_white: cell % 2 == 1,
                        position: (x, y),
                    })
                }
            }
        }

        board
    }

    fn draw(&self) {
        for (num, row) in self.grid.iter().enumerate() {
            print!("{} ", self.grid.len() - num);

            for cell in row.iter() {
                print!(
                    "{} ",
                    match cell {
                        None => "_".to_string(),
                        Some(piece) => piece.to_string(),
                    }
                );
            }
            println!();
        }

        print!("  ");
        for num in 1..=self.grid.len() {
            print!("{} ", num)
        }
        println!()
    }

    fn apply_move(&mut self, apply: &Move) {
        let mut moved_piece = self.grid[apply.old_pos.1][apply.old_pos.0].unwrap();
        moved_piece.position = apply.new_pos;
        self.grid[apply.new_pos.1][apply.new_pos.0] = Some(moved_piece);

        self.grid[apply.old_pos.1][apply.old_pos.0] = None;

        for (x, y) in apply.captures.iter() {
            self.grid[*y][*x] = None;
        }
    }

    fn applied_move(&self, apply: &Move) -> Self {
        let mut board = self.clone();

        board.apply_move(apply);

        board
    }
}

impl Piece {
    fn next_y(piece: &Self, y: Option<usize>) -> usize {
        let current_y = if let Some(y) = y { y } else { piece.position.1 };
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
    ) -> Option<Move<'a>> {
        if let (Some(other), None) = (
            board.grid[capture_pos.1][capture_pos.0],
            board.grid[new_pos.1][new_pos.0],
        ) {
            if piece.is_white != other.is_white {
                return Some(Move {
                    old_pos: piece.position,
                    new_pos,
                    piece,
                    captures: vec![capture_pos],
                });
            }
        }
        return None;
    }

    fn possible_captures(&self, board: &Board) -> Vec<Move> {
        let mut captures = Vec::new();

        if !self.is_king {
            let capture_y = Self::next_y(&self, None);
            if capture_y >= 7 || capture_y <= 0 {
                return Vec::new();
            };

            // To the left
            if self.position.0 > 1 {
                let capture_pos = (self.position.0 - 1, capture_y);
                let new_pos = (capture_pos.0 - 1, Self::next_y(&self, Some(capture_pos.1)));

                if let Some(capture) = Self::check_capture(&self, board, capture_pos, new_pos) {
                    captures.push(capture);
                }
            }

            // To the right
            if self.position.0 < 6 {
                let capture_pos = (self.position.0 + 1, capture_y);
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

    fn possible_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = self.possible_captures(board);

        if !moves.is_empty() {
            for (i, capture) in moves.clone().iter().enumerate() {
                let board_next = board.applied_move(capture);
                let piece_next = board_next.grid[capture.new_pos.1][capture.new_pos.0].unwrap();

                let mut recursive_moves = piece_next.possible_moves(&board_next);

                let mut captured_any = false;
                for recursive_move in recursive_moves.iter() {
                    if !recursive_move.captures.is_empty() {
                        captured_any = true;
                    }
                }

                let Move { captures, .. } = moves.remove(i);

                if captured_any {
                    for recursive_move in recursive_moves.iter_mut() {
                        let mut captures = captures.clone();
                        captures.append(&mut recursive_move.captures);

                        moves.push(Move {
                            old_pos: self.position,
                            new_pos: recursive_move.new_pos,
                            piece: &self,
                            captures,
                        });
                    }
                } else {
                    moves.push(Move {
                        old_pos: self.position,
                        new_pos: piece_next.position,
                        piece: &self,
                        captures: captures,
                    })
                }
            }
        } else {
            let next_y = Self::next_y(&self, None);

            // To the left
            if self.position.0 > 0 {
                let new_pos = (self.position.0 - 1, next_y);

                if board.grid[new_pos.1][new_pos.0].is_none() {
                    moves.push(Move {
                        old_pos: self.position,
                        new_pos,
                        piece: &self,
                        captures: Vec::new(),
                    });
                }
            }

            // To the right
            if self.position.0 < 7 {
                let new_pos = (self.position.0 + 1, next_y);

                if board.grid[new_pos.1][new_pos.0].is_none() {
                    moves.push(Move {
                        old_pos: self.position,
                        new_pos,
                        piece: &self,
                        captures: Vec::new(),
                    });
                }
            }
        }

        moves
    }
}
