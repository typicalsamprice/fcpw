use std::path::PrefixComponent;

use crate::bitboard::Bitboard;
use crate::piece::PieceType;
use crate::position::Position;
use crate::square::{Rank, Square};

// TODO: Maybe use NonZeroU16 to make use of NPO when using Option<Move>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    from: Square,
    to: Square,
    kind: MoveKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveKind {
    Normal,
    Castle,
    EnPassant,
    Promotion(PieceType),
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        if cfg!(feature = "strict_checks") {
            assert_ne!(from, to);
        }
        Self {
            from,
            to,
            kind: MoveKind::Normal,
        }
    }
    pub fn new_with_kind(from: Square, to: Square, kind: MoveKind) -> Self {
        if cfg!(feature = "strict_checks") {
            assert_ne!(from, to);
            if kind == MoveKind::Castle {
                assert_eq!(from.rank(), to.rank());
            } else if kind == MoveKind::EnPassant {
                let from_i = from.rank() as i32;
                let to_i = to.rank() as i32;
                let d = from_i.abs_diff(to_i);
                assert_eq!(d, 1);
            } else if let MoveKind::Promotion(promotion_type) = kind {
                assert_ne!(promotion_type, PieceType::Pawn);
                assert_ne!(promotion_type, PieceType::King);
            }
        }

        Self { from, to, kind }
    }

    // Get a `Move` from a UCI-encoded move. That is, a move that only has the `from` and `to` designations.
    // This just involves filling in the gaps
    pub fn new_from_uci(uci_str: &[u8], pos: &Position) -> Option<Self> {
        if uci_str.len() < 4 || uci_str.len() > 5 {
            return None;
        }
        let from = &uci_str[0..2];
        let to = &uci_str[2..4];

        let promo_type = if uci_str.len() == 5 {
            Some(match uci_str[4] {
                b'n' => PieceType::Knight,
                b'b' => PieceType::Bishop,
                b'r' => PieceType::Rook,
                b'q' => PieceType::Queen,
                _ => return None, // Not a valid promotion => Not a valid move.
            })
        } else {
            None
        };
        let from_sq = Square::try_from(from).ok()?;
        let to_sq = Square::try_from(to).ok()?;
        let mut kind = MoveKind::Normal;

        let mover = pos.piece_on(from_sq)?;
        if mover.kind() == PieceType::King && from_sq.distance(to_sq) == 2 {
            kind = MoveKind::Castle;
        } else if Some(to_sq) == pos.ep() && mover.kind() == PieceType::Pawn {
            kind = MoveKind::EnPassant;
        } else if to_sq.rank() == mover.color().promo_rank() && mover.kind() == PieceType::Pawn {
            kind = MoveKind::Promotion(promo_type?);
        }

        if promo_type.is_some() && kind < MoveKind::Promotion(PieceType::Pawn) {
            return None; // Malformed, cannot promote if not a promotion-type move.
        }

        Some(Self::new_with_kind(from_sq, to_sq, kind))
    }

    pub const fn from(self) -> Square {
        self.from
    }
    pub const fn to(self) -> Square {
        self.to
    }
    pub const fn kind(self) -> MoveKind {
        self.kind
    }
    pub const fn is_promo(self) -> bool {
        match self.kind() {
            MoveKind::Promotion(_) => true,
            _ => false,
        }
    }
    pub const fn get_promo(self) -> Option<PieceType> {
        match self.kind() {
            MoveKind::Promotion(t) => Some(t),
            _ => None,
        }
    }
}

pub mod generate {
    use super::*;

    pub fn pseudo_legal(pos: &Position) -> Vec<Move> {
        let mut moves = Vec::with_capacity(128);

        pawn_moves(pos, &mut moves);

        moves
    }

    // Generation helpers.

    fn pawn_moves(pos: &Position, list: &mut Vec<Move>) {
        let us = pos.to_move();
        let pawns = pos.spec(PieceType::Pawn, us);
        let potential_promotions = pawns & Bitboard::from(Rank::Seven);
        let non_promotions = pawns ^ potential_promotions;

        for p in potential_promotions {}
    }
}
