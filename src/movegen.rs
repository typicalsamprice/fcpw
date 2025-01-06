use crate::bitboard::Bitboard;
use crate::color::Color::{self, *};
use crate::piece::PieceType;
use crate::position::Position;
use crate::square::Direction::*;
use crate::square::{Rank, Square};
use crate::strict_ne;

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
        strict_ne!(from, to);
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

    pub fn pseudo_legal(pos: &Position) -> Vec<Move> {
        let mut moves = Vec::with_capacity(128);

        pawn_moves(pos, &mut moves);

        moves
    }

    // Generation helpers.

    fn pawn_moves(pos: &Position, list: &mut Vec<Move>) {
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
        for p in non_promotions {
            let up = unsafe { p.shift_unchecked(forward) };
            let w = Bitboard::from(up.shift(West));
            let e = Bitboard::from(up.shift(East));

            if bool::from(w & enemies) {
                let t = if up.shift(West) == pos.ep() {
                    MoveKind::EnPassant
                } else {
                    MoveKind::Normal
                };
                list.push(Move::new_with_kind(p, unsafe { w.lsb_unchecked() }, t));
            }
            if bool::from(e & enemies) {
                let t = if up.shift(East) == pos.ep() {
                    MoveKind::EnPassant
                } else {
                    MoveKind::Normal
                };
                list.push(Move::new_with_kind(p, unsafe { e.lsb_unchecked() }, t));
            }
        }
    }

    fn add_prom(from: Square, to: Square, list: &mut Vec<Move>) {
        for kind in PieceType::promotable() {
            list.push(Move::new_with_kind(from, to, MoveKind::Promotion(kind)));
        }
    }

    fn knight_moves(pos: &Position, list: &mut Vec<Move>) {
        let us = pos.to_move();
        let knights = pos.spec(PieceType::Knight, us);

        for k in knights {
            let movs = precompute::knight_attacks(k) & !pos.color(us);

            for m in movs {
                list.push(Move::new(k, m));
            }
        }
    }
    fn king_moves(pos: &Position, list: &mut Vec<Move>) {
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

    fn bishop_moves(pos: &Position, list: &mut Vec<Move>) {
        let us = pos.to_move();
        let bishops = pos.spec(PieceType::Bishop, us);
        let targets = !pos.color(us); // XXX Can change if not wanting captures

        for b in bishops {
            let atts = precompute::bishop_attacks(b, targets);
            for t in atts {
                list.push(Move::new(b, t));
            }
        }
    }
    fn rook_moves(pos: &Position, list: &mut Vec<Move>) {
        let us = pos.to_move();
        let rooks = pos.spec(PieceType::Rook, us);
        let targets = !pos.color(us); // XXX Can change if not wanting captures

        for r in rooks {
            let atts = precompute::rook_attacks(r, targets);
            for t in atts {
                list.push(Move::new(r, t));
            }
        }
    }
    fn queen_moves(pos: &Position, list: &mut Vec<Move>) {
        let us = pos.to_move();
        let queens = pos.spec(PieceType::Queen, us);
        let targets = !pos.color(us); // XXX Can change if not wanting captures

        for q in queens {
            let atts = precompute::queen_attacks(q, targets);
            for t in atts {
                list.push(Move::new(q, t));
            }
        }
    }
}
