use std::num::NonZeroU16;

use crate::bitboard::Bitboard;
use crate::color::Color::{self, *};
use crate::piece::PieceType;
use crate::position::Position;
use crate::square::Direction::*;
use crate::square::{Rank, Square};
use crate::strict_ne;

// Layout of Move.
// Bits 0-5: From square
// Bits 6-11: To square
// Bit 12-14 is three bits to indicate flag.
// 000 -> Normal
// 110 -> Castle
// 111 -> EP
// XYZ -> Piece of type XYZ (transmuted), with invalid types already taken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move(NonZeroU16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveKind {
    Normal,
    Castle,
    EnPassant,
    Promotion(PieceType),
}

impl Move {
    #[cfg_attr(feature = "inline", inline)]
    pub fn new(from: Square, to: Square) -> Self {
        assert_ne!(from, to);
        Self::new_with_kind(from, to, MoveKind::Normal)
    }
    #[cfg_attr(feature = "inline", inline)]
    pub fn new_with_kind(from: Square, to: Square, kind: MoveKind) -> Self {
        let squares_u16 = (from as u16) | ((to as u16) << 6);
        let flag_u16 = match kind {
            MoveKind::Promotion(PieceType::Pawn) | MoveKind::Promotion(PieceType::King) => {
                panic!("Invalid promotion type given to Move constructor")
            }
            MoveKind::Normal => 0,
            MoveKind::Castle => 0x6000,
            MoveKind::EnPassant => 0x7000,
            MoveKind::Promotion(typ) => (typ as u16) << 12,
        };
        Self(unsafe { NonZeroU16::new_unchecked(squares_u16 | flag_u16) })
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
        } else if mover.kind() == PieceType::Pawn
            && to_sq.rank() == mover.color().relative_rank(Rank::Eight)
        {
            kind = MoveKind::Promotion(promo_type?);
        }

        if promo_type.is_some() && kind < MoveKind::Promotion(PieceType::Pawn) {
            return None; // Malformed, cannot promote if not a promotion-type move.
        }

        Some(Self::new_with_kind(from_sq, to_sq, kind))
    }

    #[cfg_attr(feature = "inline", inline)]
    pub const fn from(self) -> Square {
        unsafe { std::mem::transmute((self.0.get() & 0x3f) as u8) }
    }
    #[cfg_attr(feature = "inline", inline)]
    pub const fn to(self) -> Square {
        unsafe { std::mem::transmute(((self.0.get() >> 6) & 0x3f) as u8) }
    }
    #[cfg_attr(feature = "inline", inline)]
    pub const fn kind(self) -> MoveKind {
        let bits = ((self.0.get() >> 12) & 0x7) as u8;
        match bits {
            0 => MoveKind::Normal,
            x if x >= 1 && x <= 4 => MoveKind::Promotion(unsafe { std::mem::transmute(x) }),
            6 => MoveKind::Castle,
            7 => MoveKind::EnPassant,
            _ => panic!("Illegal bit combination in 3 bits."),
        }
    }
    #[cfg_attr(feature = "inline", inline)]
    pub const fn is_promo(self) -> bool {
        match self.kind() {
            MoveKind::Promotion(_) => true,
            _ => false,
        }
    }
    #[cfg_attr(feature = "inline", inline)]
    pub const fn get_promo(self) -> Option<PieceType> {
        match self.kind() {
            MoveKind::Promotion(t) => Some(t),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveList {
    inner: [Option<Move>; 256],
    length: usize,
}

impl MoveList {
    #[cfg_attr(feature = "inline", inline)]
    pub const fn new() -> Self {
        Self {
            inner: [None; 256],
            length: 0,
        }
    }

    #[cfg_attr(feature = "inline", inline)]
    pub const fn get(&self, index: usize) -> Option<Move> {
        if index >= self.length {
            None
        } else {
            self.inner[index]
        }
    }
    #[cfg_attr(feature = "inline", inline)]
    pub const fn len(&self) -> usize {
        self.length
    }

    #[cfg_attr(feature = "inline", inline)]
    pub const fn push(&mut self, mov: Move) {
        assert!(self.length < 256);
        self.inner[self.length] = Some(mov);
        self.length += 1;
    }
    #[cfg_attr(feature = "inline", inline)]
    pub const fn remove(&mut self, index: usize) {
        assert!(index < self.length);
        self.length -= 1;
        if index < self.length {
            self.inner[index] = self.inner[self.length];
        }
    }
}

pub struct MoveListIter<'a>(std::slice::Iter<'a, Option<Move>>);

impl<'a> MoveListIter<'a> {
    #[cfg_attr(feature = "inline", inline)]
    fn new(lst: &'a MoveList) -> Self {
        Self(lst.inner[0..lst.length].iter())
    }
}

impl<'a> Iterator for MoveListIter<'a> {
    type Item = Move;
    #[cfg_attr(feature = "inline", inline)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied().flatten()
    }
}

impl<'a> IntoIterator for &'a MoveList {
    type Item = Move;
    type IntoIter = MoveListIter<'a>;
    #[cfg_attr(feature = "inline", inline)]
    fn into_iter(self) -> Self::IntoIter {
        MoveListIter::new(self)
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prom_s = self
            .get_promo()
            .map_or_else(|| String::new(), |pt| format!("{pt}"));
        write!(f, "{}{}{}", self.from(), self.to(), prom_s)
    }
}

pub mod generate {
    use crate::{position::CastleFlag, precompute};

    use super::*;

    #[cfg_attr(feature = "inline-aggressive", inline)]
    pub fn pseudo_legal(pos: &Position) -> MoveList {
        let mut moves = MoveList::new();

        pawn_moves(pos, &mut moves);
        knight_moves(pos, &mut moves);
        //all_sliders_at_once(pos, &mut moves);
        bishop_moves(pos, &mut moves);
        rook_moves(pos, &mut moves);
        queen_moves(pos, &mut moves);
        king_moves(pos, &mut moves);

        moves
    }

    #[cfg_attr(feature = "inline", inline)]
    pub fn legal(pos: &Position) -> MoveList {
        let mut moves = pseudo_legal(pos);
        prune_to_legal(pos, &mut moves);
        moves
    }

    #[cfg_attr(feature = "inline-aggressive", inline)]
    fn prune_to_legal(pos: &Position, list: &mut MoveList) {
        let mut i = 0;
        let us = pos.to_move();
        let king = pos.king(us);
        // TODO list.filter(...)
        while i < list.len() {
            // SAFETY: Cannot be none, since i < length
            let m = unsafe { list.get(i).unwrap_unchecked() };
            if (m.from() == king
                || pos.blockers(us).has(m.from())
                || m.kind() == MoveKind::EnPassant
                || pos.in_check())
                && !pos.is_legal(m)
            {
                list.remove(i);
                continue;
            }
            i += 1;
        }
    }

    // Generation helpers.
    fn pawn_moves(pos: &Position, list: &mut MoveList) {
        let us = pos.to_move();

        let enemies = pos.color(!us) | Bitboard::from(pos.ep());
        let empty = !pos.all();

        let pawns = pos.spec(PieceType::Pawn, us);
        let potential_promotions = pawns & Bitboard::from(us.relative_rank(Rank::Seven));
        let non_promotions = pawns ^ potential_promotions;

        let third_rank = Bitboard::from(us.relative_rank(Rank::Three));
        let forward = if us == White { North } else { South };

        // All promotions
        for p in potential_promotions {
            unsafe {
                let up = p.shift_unchecked(forward);
                if pos.empty(up) {
                    add_prom(p, up, list);
                }

                let proms = Bitboard::from([up.shift(East), up.shift(West)]) & enemies;
                for dest in proms {
                    add_prom(p, dest, list);
                }
            }
        }

        // Pushes
        let one_ups = (non_promotions << forward) & empty;
        let two_ups = ((one_ups & third_rank) << forward) & empty;

        for p in one_ups {
            list.push(Move::new(unsafe { p.shift_unchecked(!forward) }, p));
        }
        for p in two_ups {
            list.push(Move::new(
                unsafe { p.shift_unchecked(!forward).shift_unchecked(!forward) },
                p,
            ));
        }

        // Captures
        let up_east = non_promotions.shift(forward).shift(East) & enemies;
        let up_west = non_promotions.shift(forward).shift(West) & enemies;

        for x in up_east {
            let f = unsafe { x.shift_unchecked(forward.not()).shift_unchecked(West) };
            let t = if Some(x) == pos.ep() {
                MoveKind::EnPassant
            } else {
                MoveKind::Normal
            };
            list.push(Move::new_with_kind(f, x, t));
        }
        for x in up_west {
            let f = unsafe { x.shift_unchecked(forward.not()).shift_unchecked(East) };
            let t = if Some(x) == pos.ep() {
                MoveKind::EnPassant
            } else {
                MoveKind::Normal
            };
            list.push(Move::new_with_kind(f, x, t));
        }
    }

    fn add_prom(from: Square, to: Square, list: &mut MoveList) {
        for kind in PieceType::promotable() {
            list.push(Move::new_with_kind(from, to, MoveKind::Promotion(kind)));
        }
    }

    fn knight_moves(pos: &Position, list: &mut MoveList) {
        let us = pos.to_move();
        let knights = pos.spec(PieceType::Knight, us);

        for k in knights {
            let movs = precompute::knight_attacks(k) & !pos.color(us);

            for m in movs {
                list.push(Move::new(k, m));
            }
        }
    }
    fn king_moves(pos: &Position, list: &mut MoveList) {
        let us = pos.to_move();
        let king = pos.king(us);

        let movs = precompute::king_attacks(king) & !pos.color(us);

        for m in movs {
            list.push(Move::new(king, m));
        }

        for cf in CastleFlag::variants_for(us) {
            if pos.has_castle(cf) && pos.can_castle(cf) {
                list.push(Move::new_with_kind(
                    cf.from_square(),
                    cf.to_square(),
                    MoveKind::Castle,
                ));
            }
        }
    }

    fn bishop_moves(pos: &Position, list: &mut MoveList) {
        let us = pos.to_move();
        let bishops = pos.spec(PieceType::Bishop, us);
        let targets = !pos.color(us); // XXX Can change if not wanting captures

        for b in bishops {
            let atts = precompute::bishop_attacks(b, pos.all()) & targets;
            for t in atts {
                list.push(Move::new(b, t));
            }
        }
    }
    fn rook_moves(pos: &Position, list: &mut MoveList) {
        let us = pos.to_move();
        let rooks = pos.spec(PieceType::Rook, us);
        let targets = !pos.color(us); // XXX Can change if not wanting captures

        for r in rooks {
            let atts = precompute::rook_attacks(r, pos.all()) & targets;
            for t in atts {
                list.push(Move::new(r, t));
            }
        }
    }
    fn queen_moves(pos: &Position, list: &mut MoveList) {
        let us = pos.to_move();
        let queens = pos.spec(PieceType::Queen, us);
        let targets = !pos.color(us); // XXX Can change if not wanting captures

        for q in queens {
            let atts = precompute::queen_attacks(q, pos.all()) & targets;
            for t in atts {
                list.push(Move::new(q, t));
            }
        }
    }

    fn all_sliders_at_once(pos: &Position, list: &mut MoveList) {
        let us = pos.to_move();
        let queens = pos.spec(PieceType::Queen, us);
        let bishops = pos.spec(PieceType::Bishop, us);
        let rooks = pos.spec(PieceType::Rook, us);
        let targets = !pos.color(us); // XXX Can change if not wanting captures

        for b in bishops | queens {
            let atts = precompute::bishop_attacks(b, pos.all()) & targets;
            for t in atts {
                list.push(Move::new(b, t));
            }
        }
        for r in rooks | queens {
            let atts = precompute::rook_attacks(r, pos.all()) & targets;
            for t in atts {
                list.push(Move::new(r, t));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MoveKind::*;
    use PieceType::*;
    use Square::*;

    #[test]
    fn created_moves_have_expected_squares() {
        let m1 = Move::new(A1, A2);
        assert_eq!(m1.from(), A1);
        assert_eq!(m1.to(), A2);
        assert_eq!(m1.kind(), Normal);

        let m2 = Move::new(A5, H8);
        assert_eq!(m2.from(), A5);
        assert_eq!(m2.to(), H8);
        assert_eq!(m2.kind(), Normal);
    }

    #[test]
    fn promotion_type_encodes() {
        let m1 = Move::new_with_kind(A1, A2, Promotion(Knight));
        assert_eq!(m1.from(), A1);
        assert_eq!(m1.to(), A2);
        assert_eq!(m1.kind(), Promotion(Knight));

        let m2 = Move::new_with_kind(A1, E8, Promotion(Queen));
        assert_eq!(m2.from(), A1);
        assert_eq!(m2.to(), E8);
        assert_eq!(m2.kind(), Promotion(Queen));

        assert_eq!(m1.is_promo(), true);
        assert_eq!(m2.is_promo(), true);

        assert_eq!(m1.get_promo(), Some(Knight));
        assert_eq!(m2.get_promo(), Some(Queen));
    }

    #[test]
    fn kind_encodes() {
        let m1 = Move::new(A2, A5);
        let m2 = Move::new_with_kind(A2, A5, Normal);
        let m3 = Move::new_with_kind(A1, A7, Castle);
        let m4 = Move::new_with_kind(A1, F4, EnPassant);
        let m5 = Move::new_with_kind(A1, F4, Promotion(Queen));

        assert_eq!(m1.kind(), Normal);
        assert_eq!(m2.kind(), Normal);
        assert_eq!(m3.kind(), Castle);
        assert_eq!(m4.kind(), EnPassant);
        assert_eq!(m5.kind(), Promotion(Queen));
    }
}
