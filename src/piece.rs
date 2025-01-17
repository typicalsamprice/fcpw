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
    pub const fn new(kind: PieceType, color: Color) -> Self {
        Self { kind, color }
    }
    pub const fn kind(&self) -> PieceType {
        self.kind
    }
    pub const fn color(&self) -> Color {
        self.color
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
