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

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Piece {
    kind: PieceType,
    color: Color,
}

impl Piece {
    pub const fn kind(&self) -> PieceType {
        self.kind
    }
    pub const fn color(&self) -> Color {
        self.color
    }
}
