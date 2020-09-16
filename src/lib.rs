#![feature(exclusive_range_pattern)]

pub mod interface;
pub use interface::Checkers;

#[derive(Clone)]
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
            grid: [[Cell::Empty; 8]; 8],
        };

        for y in 0..8usize {
            for x in 0..8usize {
                let cell = template[y][x];
                if cell != 0 {
                    board.grid[y][x] = Cell::Piece(Piece {
                        is_king: cell > 2,
                        is_white: cell % 2 == 1,
                        position: (x, y),
                    })
                }
            }
        }

        board
    }

    fn draw(&self, draw_numbers: bool) {
        for (num, row) in self.grid.iter().enumerate() {
            if draw_numbers {
                print!("{} ", self.grid.len() - num)
            }

            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }

        if draw_numbers {
            print!("  ");
            for num in 1..=self.grid.len() {
                print!("{} ", num)
            }
            println!()
        }
    }

    fn apply_move(&mut self, apply: Move) {
        self.grid[apply.new_pos.1][apply.new_pos.0] = self.grid[apply.old_pos.1][apply.old_pos.0];
        self.grid[apply.old_pos.1][apply.old_pos.0] = Cell::Empty;

        for (x, y) in apply.captures.iter() {
            self.grid[*y][*x] = Cell::Empty;
        }
    }

    fn applied_move(&self, apply: Move) -> Self {
        let mut board = self.clone();

        board.apply_move(apply);

        board
    }
}

impl Piece {
    fn possible_positions(&self) -> Vec<(usize, usize)> {
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

        possible_positions
    }

    fn possible_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();

        for next_position in self.possible_positions().iter() {
            let to_left = (next_position.0 as i8 - self.position.0 as i8) < 0;

            match board.grid[next_position.1][next_position.0] {
                Cell::Empty => moves.push(Move {
                    old_pos: self.position,
                    new_pos: *next_position,
                    piece: &self,
                    captures: vec![],
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
                                next_position.1 - 1
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
                                captures: vec![(next_position.0, next_position.1)],
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
