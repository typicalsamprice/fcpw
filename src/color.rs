use std::ops::Not;

use crate::square::Rank;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[inline]
    pub const fn relative_rank(self, rank: Rank) -> Rank {
        match self {
            Color::White => rank,
            Color::Black => unsafe { std::mem::transmute(7 - rank as u8) },
        }
    }
}

impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
