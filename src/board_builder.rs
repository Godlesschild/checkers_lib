use crate::{Board, Piece, Position};

pub struct BoardBuilder {
    board: Board,
    white_pieces: u8,
    black_pieces: u8,
}

impl BoardBuilder {
    pub fn empty() -> Self {
        Self {
            board: Board {
                grid: [[None; 8]; 8],
            },
            white_pieces: 0,
            black_pieces: 0,
        }
    }

    pub fn default() -> Self {
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

        BoardBuilder::try_from_template(template).unwrap()
    }

    pub fn build(self) -> Board {
        self.board
    }

    pub fn try_from_template(template: [[u8; 8]; 8]) -> Result<Self, &'static str> {
        let mut builder = Self {
            board: Board {
                grid: [[None; 8]; 8],
            },
            white_pieces: 0,
            black_pieces: 0,
        };

        for y in 0..8usize {
            for x in 0..8usize {
                let cell = template[y][x];
                if cell != 0 {
                    let position = Position::from_coordinates((x, y))?;

                    let is_king = cell > 2;
                    let is_white = cell % 2 == 1;

                    builder.try_insert(position, is_king, is_white)?;
                }
            }
        }

        Ok(builder)
    }

    pub fn try_insert(
        &mut self,
        position: Position,
        is_king: bool,
        is_white: bool,
    ) -> Result<(), &'static str> {
        let (x, y) = position.as_coordinates();

        if is_white && self.white_pieces == 12 {
            return Err("cant have more than 12 white pieces on board");
        } else if !is_white && self.black_pieces == 12 {
            return Err("cant have more than 12 black pieces on board");
        }

        if !self.board.grid[y][x].is_none() {
            Err("cant insert into occupied tile")
        } else {
            self.board.grid[y][x] = Some(Piece {
                is_king,
                is_white,
                position,
            });

            if is_white {
                self.white_pieces += 1;
            } else {
                self.black_pieces += 1;
            }

            Ok(())
        }
    }

    pub fn try_remove(&mut self, position: Position) -> Result<(), &'static str> {
        let (x, y) = position.as_coordinates();

        if self.board.grid[y][x].is_none() {
            Err("cant remove from empty tile")
        } else {
            let Piece { is_white, .. } = self.board.grid[y][x].unwrap();

            self.board.grid[y][x] = None;

            if is_white {
                self.white_pieces -= 1;
            } else {
                self.black_pieces -= 1;
            }

            Ok(())
        }
    }

    pub fn try_replace(
        &mut self,
        position: Position,
        is_king: bool,
        is_white: bool,
    ) -> Result<(), &'static str> {
        self.try_remove(position)?;
        self.try_insert(position, is_king, is_white)?;

        Ok(())
    }
}
