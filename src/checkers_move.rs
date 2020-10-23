use crate::Piece;

#[derive(Clone)]
pub struct CheckersMove<'a> {
    pub old_pos: (usize, usize),
    pub new_pos: (usize, usize),
    pub piece: &'a Piece,
    pub captures: Vec<(usize, usize)>,
}

impl<'a> std::fmt::Display for CheckersMove<'a> {
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
