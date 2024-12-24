use std::ops::Not;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Color {
    White,
    Black,
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
