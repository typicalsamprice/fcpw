use std::ops::Not;

use crate::square::{Direction, Rank};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[cfg_attr(feature = "inline", inline)]
    pub const fn relative_rank(self, rank: Rank) -> Rank {
        match self {
            Color::White => rank,
            Color::Black => unsafe { std::mem::transmute(7 - rank as u8) },
        }
    }

    #[cfg_attr(feature = "inline", inline)]
    pub const fn forward(self) -> Direction {
        match self {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        }
    }

    #[cfg_attr(feature = "inline", inline)]
    pub const fn not(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Not for Color {
    type Output = Self;
    #[cfg_attr(feature = "inline", inline)]
    fn not(self) -> Self::Output {
        self.not()
    }
}
