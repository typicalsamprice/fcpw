use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::piece::{Piece, PieceType};
use crate::square::{File, Rank, Square};

use std::ptr::NonNull;

#[derive(Debug)]
pub struct Position {
    to_move: Color,
    moves: i32,

    colors: [Bitboard; 2],
    pieces: [Bitboard; 6],
    board: [Option<Piece>; 64],

    state: NonNull<State>,
}

#[derive(Debug)]
pub struct State {
    checkers: Bitboard,
    pinners: [Bitboard; 2],
    blockers: [Bitboard; 2],
    captured: Option<Piece>,
    en_passant: Option<Square>,

    castle_rights: u8,

    halfmoves: i32,

    previous: Option<NonNull<State>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleFlag {
    WhiteShort,
    WhiteLong,
    WhiteAll,
    BlackShort,
    BlackLong,
    BlackAll,
    All,
}

impl From<CastleFlag> for u8 {
    fn from(value: CastleFlag) -> Self {
        match value {
            CastleFlag::All => 0xF,
            CastleFlag::WhiteShort => 0x1,
            CastleFlag::WhiteLong => 0x2,
            CastleFlag::WhiteAll => 0x1 | 0x2,
            CastleFlag::BlackShort => 0x4,
            CastleFlag::BlackLong => 0x8,
            CastleFlag::BlackAll => 0x4 | 0x8,
        }
    }
}

impl Position {
    // Misc data pulls
    pub const fn to_move(&self) -> Color {
        self.to_move
    }
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
    pub fn pieces_list(&self, ts: &[PieceType]) -> Bitboard {
        let mut res: Bitboard = 0.into();
        for t in ts {
            res |= self.pieces(*t);
        }
        res
    }
    pub fn spec(&self, t: PieceType, c: Color) -> Bitboard {
        self.pieces(t) & self.color(c)
    }
    pub fn spec_list(&self, ts: &[PieceType], c: Color) -> Bitboard {
        self.pieces_list(ts) & self.color(c)
    }

    pub const fn piece_on(&self, s: Square) -> Option<Piece> {
        self.board[s as usize]
    }
    // Castling
    pub fn has_castle(&self, cf: CastleFlag) -> bool {
        let cf_u8: u8 = cf.into();
        self.state().castle_rights & cf_u8 == cf_u8
    }
    pub fn can_castle(&self, cf: CastleFlag) -> bool {
        if cfg!(feature = "strict-checks") && !self.has_castle(cf) {
            return false;
        }
        todo!()
    }

    // State access. First is not public for "obvious" reasons
    const fn state(&self) -> &State {
        // SAFETY: This is always non-null, and only accessed from here. Also is created from correct pointer, and so is not misaligned.
        unsafe { self.state.as_ref() }
    }
    pub const fn ep(&self) -> Option<Square> {
        self.state().en_passant
    }
    pub const fn checkers(&self) -> Bitboard {
        self.state().checkers
    }
    pub const fn pinners(&self, color: Color) -> Bitboard {
        self.state().pinners[color as usize]
    }
    pub const fn blockers(&self, color: Color) -> Bitboard {
        self.state().blockers[color as usize]
    }
    pub const fn rule50(&self) -> i32 {
        self.state().halfmoves
    }
}
