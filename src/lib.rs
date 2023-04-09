//! Library for checkers game generation

// #![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
// #![warn(missing_docs)]

#![doc(html_playground_url = "https://play.rust-lang.org")]

mod board_builder;
pub use board_builder::BoardBuilder;

mod board;
pub use board::Board;

mod piece;
pub use piece::Piece;

mod checkers_move;
pub use checkers_move::CheckersMove;

pub mod position;
pub use position::Position;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("")]
    Position(#[from] position::Error),

    #[error("")]
    Rules(#[from] RulesError),
}

#[derive(thiserror::Error, Debug)]
pub enum RulesError {
    #[error("tile {0:?} is empty")]
    Empty(Position),

    #[error("tile {0:?} is empty")]
    Occupied(Position),

    #[error("tried to go over {} color limit", if *is_white {"white"} else {"black"})]
    ColorLimit { is_white: bool },

    #[error(
        "{} piece at {} tried to capture piece at {} of same color", 
        if *is_white {"white"} else {"black"}, 
        capturing_pos, 
        captured_pos
    )]
    SameColorCapture {
        capturing_pos: Position,
        captured_pos: Position,
        is_white: bool,
    },

    #[error("piece at {0} is not a king")]
    NotKing(Position)
}
