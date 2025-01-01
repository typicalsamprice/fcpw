use std::ops::Not;

use crate::square::Rank;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[inline]
    pub const fn back_rank(self) -> Rank {
        match self {
            Color::White => Rank::One,
            Color::Black => Rank::Eight,
        }
    }
    #[inline]
    pub const fn promo_rank(self) -> Rank {
        match self {
            Color::White => Rank::Eight,
            Color::Black => Rank::One,
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
