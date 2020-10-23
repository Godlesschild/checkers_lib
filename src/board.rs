use crate::{CheckersMove, Piece};

#[derive(Clone)]
pub struct Board {
    pub grid: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
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

    pub fn from_template(template: [[u8; 8]; 8]) -> Self {
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

    pub fn draw(&self) {
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

    pub fn possible_moves(&self, current_white: bool) -> Vec<CheckersMove> {
        let mut moves = Vec::new();

        for row in self.grid.iter() {
            for cell in row.iter() {
                if let Some(piece) = cell {
                    if piece.is_white == current_white {
                        moves.append(&mut piece.possible_moves(&self))
                    }
                }
            }
        }

        moves
    }

    pub fn allowed_moves(&self, current_white: bool) -> Vec<CheckersMove> {
        let possible_moves = self.possible_moves(current_white);
        let mut allowed_moves = Vec::new();

        let mut captured_any = false;
        for i in possible_moves.iter() {
            if !i.captures.is_empty() {
                captured_any = true;
            }
        }

        if captured_any {
            for i in possible_moves.into_iter() {
                if !i.captures.is_empty() {
                    allowed_moves.push(i)
                }
            }
            allowed_moves
        } else {
            possible_moves
        }
    }

    pub fn apply_move(&mut self, apply: &CheckersMove) {
        let mut moved_piece = self.grid[apply.old_pos.1][apply.old_pos.0].unwrap();
        moved_piece.position = apply.new_pos;
        self.grid[apply.new_pos.1][apply.new_pos.0] = Some(moved_piece);

        self.grid[apply.old_pos.1][apply.old_pos.0] = None;

        for (x, y) in apply.captures.iter() {
            self.grid[*y][*x] = None;
        }
    }

    pub fn applied_move(&self, apply: &CheckersMove) -> Self {
        let mut board = self.clone();

        board.apply_move(apply);

        board
    }
}
