use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::piece::{Piece, PieceType};

#[derive(Debug)]
pub struct Position {
    to_move: Color,
    colors: [Bitboard; 2],
    pieces: [Bitboard; 6],
    board: [Option<Piece>; 64],
}

impl Position {
    // Bitboard pulling
    pub fn all(&self) -> Bitboard {
        self.colors[0] | self.colors[1]
    }
    pub fn color(&self, c: Color) -> Bitboard {
        self.colors[c as usize]
    }
    pub fn pieces(&self, t: PieceType) -> Bitboard {
        self.pieces[t as usize]
    }
}
