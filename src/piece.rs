use std::num::NonZeroU8;

use crate::color::Color;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

// Bits 0-2 => Enough to give pieces types. Specifically, the values 1-7 are held, and we subtract one on conversion to keep nonzero-ness.
// Then, the fourth bit is for color!
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Piece(NonZeroU8);

impl Piece {
    pub const fn new(kind: PieceType, color: Color) -> Self {
        let inner = (kind as u8 + 1) | ((color as u8) << 3);
        Self(unsafe { NonZeroU8::new_unchecked(inner) })
    }
    pub const fn kind(&self) -> PieceType {
        unsafe { std::mem::transmute((self.0.get() & 7) - 1) }
    }
    pub const fn color(&self) -> Color {
        unsafe { std::mem::transmute(self.0.get() >> 3) }
    }
}

impl PieceType {
    pub const fn promotable() -> [Self; 4] {
        use PieceType::*;
        [Knight, Bishop, Rook, Queen]
    }
}

impl From<PieceType> for char {
    fn from(value: PieceType) -> Self {
        use PieceType::*;
        match value {
            Pawn => 'p',
            Knight => 'n',
            Bishop => 'b',
            Rook => 'r',
            Queen => 'q',
            King => 'k',
        }
    }
}
impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        let s = char::from(value.kind());
        match value.color() {
            Color::Black => s,
            Color::White => (s as u8 - 32) as char,
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let kind = match value.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => Err(())?,
        };
        let col = match value.is_lowercase() {
            false => Color::White,
            true => Color::Black,
        };

        Ok(Self::new(kind, col))
    }
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
