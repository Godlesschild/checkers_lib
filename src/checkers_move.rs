use crate::Piece;
use crate::Position;
use std::fmt::Write;

#[derive(Clone)]
pub struct CheckersMove<'a> {
    pub old_pos: Position,
    pub new_pos: Position,
    pub piece: &'a Piece,
    pub captures: Vec<Position>,
}

impl<'a> CheckersMove<'a> {
    fn to_notation(&self, collapsed: bool) -> String {
        if self.captures.is_empty() {
            format!(
                "{}-{}",
                self.old_pos.as_notation(),
                self.new_pos.as_notation()
            )
        } else {
            if collapsed {
                format!(
                    "{}x{}",
                    self.old_pos.as_notation(),
                    self.new_pos.as_notation()
                )
            } else {
                let mut buf = String::new();
                write!(buf, "{}x", self.old_pos.as_notation()).unwrap();

                for capture in self.captures.iter() {
                    write!(buf, "{}x", capture.as_notation()).unwrap();
                }

                write!(buf, "{}", self.new_pos.as_notation()).unwrap();

                buf
            }
        }
    }
}

impl<'a> std::fmt::Display for CheckersMove<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_notation(true))
    }
}
