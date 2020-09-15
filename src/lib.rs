#![feature(exclusive_range_pattern)]

pub mod logic;

pub use logic::Checkers;
use logic::Move;

pub struct Board {
    grid: [[Cell; 8]; 8],
}

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Piece(Piece),
}

#[derive(Copy, Clone)]
pub struct Piece {
    is_king: bool,
    is_white: bool,
    position: (usize, usize),
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            grid: [[Cell::Empty; 8]; 8],
        };
        let temp_grid: [[u8; 8]; 8] = [
            [0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 1, 0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0],
        ];

        for y in 0..8usize {
            for x in 0..8usize {
                if temp_grid[y][x] == 1 {
                    board.grid[y][x] = Cell::Piece(Piece {
                        is_king: false,
                        is_white: y > 3,
                        position: (x, y),
                    })
                }
            }
        }

        board
    }

    pub fn draw(&self) {
        for row in self.grid.iter() {
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => "_".to_string(),
                Cell::Piece(piece) => piece.to_string(),
            }
        )
    }
}

impl Piece {
    fn possible_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();

        let next_y = if self.is_white && !self.is_king {
            if self.position.1 > 0 {
                self.position.1 - 1
            } else {
                return Vec::new(); //* TEMPORARY before adding kings
            }
        } else {
            self.position.1 + 1
        };

        let mut possible_positions = Vec::new();
        if let (0..8, 0..8) = (self.position.0 + 1, next_y) {
            // right
            possible_positions.push((self.position.0 + 1, next_y))
        }
        if let (0..8, 0..8) = (self.position.0 as i8 - 1, next_y) {
            // left
            possible_positions.push((self.position.0 - 1, next_y))
        }

        for next_position in possible_positions.iter() {
            let to_left = (next_position.0 as i8 - self.position.0 as i8) < 0;

            match board.grid[next_position.1][next_position.0] {
                Cell::Empty => moves.push(Move {
                    old_pos: self.position,
                    new_pos: *next_position,
                    piece: &self,
                    capture: None,
                }),

                Cell::Piece(Piece { is_white, .. }) => {
                    // Check if it's possible to capture

                    if (self.is_white && next_position.1 == 0)
                        | (self.is_white && next_position.1 == 8)
                        | (is_white == self.is_white)
                    {
                        break;
                    };

                    let after_capture = (
                        if to_left {
                            if next_position.0 > 0 {
                                next_position.0 - 1
                            } else {
                                break;
                            }
                        } else {
                            next_position.0 + 1
                        },
                        if self.is_white {
                            if next_position.1 > 0 {
                                next_position.0 - 1
                            } else {
                                break;
                            }
                        } else {
                            next_position.1 + 1
                        },
                    );

                    if let (0..8, 0..8) = after_capture {
                        if let Cell::Empty = board.grid[after_capture.1][after_capture.0] {
                            moves.push(Move {
                                old_pos: self.position,
                                new_pos: after_capture,
                                piece: &self,
                                capture: Some((next_position.0, next_position.1)),
                            })
                        }
                    }
                }
            }
        }

        // TODO Add recursion to check for multiple captures

        // TODO Add kings

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
