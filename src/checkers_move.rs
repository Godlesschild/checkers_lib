use crate::Piece;
use crate::Position;

#[derive(Clone)]
pub struct CheckersMove<'a> {
    pub old_pos: Position,
    pub new_pos: Position,
    pub piece: &'a Piece,
    pub captures: Vec<Position>,
}

impl<'a> std::fmt::Display for CheckersMove<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let old_pos = self.old_pos.as_coordinates();
        let new_pos = self.new_pos.as_coordinates();
        write!(
            f,
            "({};{}) to ({};{})",
            old_pos.0 + 1,
            8 - old_pos.1,
            new_pos.0 + 1,
            8 - new_pos.1
        )
    }
}
