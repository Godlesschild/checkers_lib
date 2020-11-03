use crate::{CheckersMove, Piece};

#[derive(Clone)]
pub struct Board {
    pub(crate) grid: [[Option<Piece>; 8]; 8],
}

impl Board {
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

    pub fn legal_moves(&self, current_white: bool) -> Vec<CheckersMove> {
        let possible_moves = self.possible_moves(current_white);
        let mut legal_moves = Vec::new();

        let mut captured_any = false;
        for i in possible_moves.iter() {
            if !i.captures.is_empty() {
                captured_any = true;
            }
        }

        if captured_any {
            for i in possible_moves.into_iter() {
                if !i.captures.is_empty() {
                    legal_moves.push(i)
                }
            }
            legal_moves
        } else {
            possible_moves
        }
    }

    pub fn apply_move(&mut self, apply: &CheckersMove) {
        let (old_x, old_y) = apply.old_pos.as_coordinates();
        let (new_x, new_y) = apply.new_pos.as_coordinates();

        let mut moved_piece = self.grid[old_y][old_x].unwrap();
        moved_piece.position = apply.new_pos;
        self.grid[new_y][new_x] = Some(moved_piece);

        self.grid[old_y][old_x] = None;

        for position in apply.captures.iter() {
            let (x, y) = position.as_coordinates();
            self.grid[y][x] = None;
        }
    }

    pub fn applied_move(&self, apply: &CheckersMove) -> Self {
        let mut board = self.clone();

        board.apply_move(apply);

        board
    }
}
