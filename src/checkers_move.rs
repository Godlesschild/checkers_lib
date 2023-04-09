use itertools::Itertools;

use crate::Piece;
use crate::Position;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Write};

#[derive(Clone, PartialEq)]
pub struct CheckersMove {
    pub(crate) old: Piece,
    pub(crate) new: Piece,
    pub(crate) captures: HashSet<Position>,
}

impl CheckersMove {
    #[must_use]
    pub fn arbitrary(old: Piece, new: Piece, captures: HashSet<Position>) -> Self {
        CheckersMove { old, new, captures }
    }

    #[must_use]
    pub fn captures(&self) -> &HashSet<Position> {
        &self.captures
    }

    #[must_use]
    pub fn old_piece(&self) -> Piece {
        self.old
    }

    #[must_use]
    pub fn new_piece(&self) -> Piece {
        self.new
    }

    /// Returns a string representation of a move.
    ///
    /// Also see [`Position`][0]'s `Display` impl.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use checkers_lib::*;
    /// #
    /// let piece = Piece { is_king: false, is_white: true, position: 29.try_into()? };
    /// let board = BoardBuilder::empty().try_insert(piece)?.build();
    /// let test_move = &board.all_possible_moves(true)[0];
    ///
    /// assert_eq!(test_move.to_string::<u8>(false), "29-25");
    /// assert_eq!(test_move.to_string::<(usize, usize)>(false), "(0, 7)-(1, 6)");
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [0]: crate::position::Position::fmt
    #[must_use]
    pub fn to_string<T>(&self, collapsed: bool) -> String
    where
        T: From<Position> + Debug,
    {
        let old: T = self.old.position.into();
        let new: T = self.new.position.into();

        if self.captures.is_empty() {
            return format!("{old:?}-{new:?}");
        }

        if collapsed {
            return format!("{old:?}x{new:?}");
        }

        let mut buf = String::new();
        write!(buf, "{old:?}x").unwrap();

        for capture in &self.captures {
            write!(buf, "{:?}x", Into::<T>::into(*capture)).unwrap();
        }

        write!(buf, "{new:?}").unwrap();

        buf
    }
}

impl Debug for CheckersMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = crate::BoardBuilder::empty().try_insert(self.old).unwrap();

        for capture_pos in &self.captures {
            builder = builder
                .try_insert(Piece::new(false, !self.old.is_white, *capture_pos))
                .unwrap();
        }

        let board_from = builder.build();

        let board_to = crate::BoardBuilder::empty()
            .try_insert(self.new)
            .unwrap()
            .build();

        writeln!(f)?;
        writeln!(
            f,
            "      Board 1     -->      Board 2       |  {} captures",
            self.captures.len()
        )?;

        for (a, b) in format!("{board_from}")
            .lines()
            .interleave(format!("{board_to:?}").lines())
            .tuples::<(_, _)>()
        {
            writeln!(f, "{a} |  {b} |")?;
        }
        writeln!(f)
    }
}

impl Display for CheckersMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string::<u8>(false))
    }
}
