use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::movegen::{Move, MoveKind};
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Evaluation {
    Stalemate,
    Score(f32),
    MateIn(i32), // Positive for white, negative for black
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
        if cfg!(feature = "strict_checks") && !self.has_castle(cf) {
            return false;
        }
        todo!()
    }

    // State access. First two are not public for "obvious" reasons. Namely, we don't want a reference that can become invalidated.
    const fn state(&self) -> &State {
        // SAFETY: This is always non-null, and only accessed from here. Also is created from correct pointer, and so is not misaligned.
        unsafe { self.state.as_ref() }
    }
    const unsafe fn state_mut(&mut self) -> &mut State {
        // SAFETY: Up to the caller
        unsafe { self.state.as_mut() }
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

    // Move related
    pub fn is_legal(&self, mov: Move) -> bool {
        if cfg!(feature = "strict_checks") && !self.is_pseudo_legal(mov) {
            return false;
        }
        todo!();
    }
    pub fn is_pseudo_legal(&self, mov: Move) -> bool {
        let src = mov.from();
        let tar = mov.to();
        let kind = mov.kind();

        if src == tar {
            return false;
        }

        let us = self.to_move();
        let them = !us;
        let our_pieces = self.color(us);
        let their_pieces = self.color(!us);

        let Some(mover) = self.piece_on(src) else {
            return false; // No piece!
        };

        if mover.color() != us {
            return false;
        }

        let opt_taken = self.piece_on(tar);

        if opt_taken.map(|p| p.color()) == Some(us) {
            return false; // Cannot take own piece.
        }

        if kind == MoveKind::Castle {
            let dist = src.distance(tar);
            if mover.kind() != PieceType::King {
                return false;
            }
            if dist != 2 {
                return false; // Castling is always a 2-square king move
            }
            let between = Bitboard::interval(src, tar) | Bitboard::from(tar);
        }

        true
    }
    pub fn make_move(&mut self, mov: Move) {
        todo!()
    }
    pub fn unmake_move(&mut self, mov: Move) {
        todo!()
    }

    // Evalutation
    pub fn evaluate(&self) -> Evaluation {
        todo!()
    }

    // Rest private helpers
}
