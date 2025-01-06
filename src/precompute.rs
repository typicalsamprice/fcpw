use std::sync::OnceLock;

// TODO Precompute elements
// - Piece moves, including sliding pieces (start with rays for simplicity, transition to magic bitboards if required)
use crate::bitboard::Bitboard;
use crate::color::Color::{self, *};
use crate::square::{Direction, Square};

static IS_INIT: OnceLock<bool> = OnceLock::new();

const BB_ZERO: Bitboard = Bitboard::new(0);
const BB_FULL: Bitboard = Bitboard::new(!0u64);

static mut BB_RAYS: [[Bitboard; 8]; 64] = [[BB_ZERO; 8]; 64];
static mut BB_LINES: [[Bitboard; 64]; 64] = [[BB_ZERO; 64]; 64];

static mut ATT_KNIGHT: [Bitboard; 64] = [BB_ZERO; 64];
static mut ATT_KING: [Bitboard; 64] = [BB_ZERO; 64];
static mut ATT_PAWNS: [[Bitboard; 2]; 64] = [[BB_ZERO; 2]; 64];

pub fn initialize() {
    if IS_INIT.get() == Some(&true) {
        return;
    }

    // Setup for ray/line caching
    for square in BB_FULL {
        for d in Direction::all() {
            let mut s = Bitboard::from(square);
            let mut r = BB_ZERO;
            while bool::from(s) {
                s <<= d;
                r |= s;
            }
            unsafe { BB_RAYS[square as usize][d as usize] = r };
        }

        for other in BB_FULL {
            // If it's not on the same line OR the entry is nonzero, we can continue forward.
            if !square.same_line(other)
                || bool::from(unsafe { BB_LINES[square as usize][other as usize] })
            {
                continue;
            }

            // SAFETY: We know they are on the same line, so it cannot be `None`.
            let a = unsafe { square.dir_to(other).unwrap_unchecked() };
            let b = unsafe { other.dir_to(square).unwrap_unchecked() };

            unsafe {
                let line = BB_RAYS[square as usize][a as usize]
                    | BB_RAYS[square as usize][b as usize]
                    | Bitboard::from(square);
                BB_LINES[square as usize][other as usize] = line;
                BB_LINES[other as usize][square as usize] = line;
            }
        }
    }

    // Setup for king + pawn moves
    for square in BB_FULL {
        // Pawns first
        let s = Bitboard::from(square);
        let sides = (s << Direction::West) | (s << Direction::East);

        unsafe {
            ATT_PAWNS[square as usize][White as usize] = sides << Direction::North;
            ATT_PAWNS[square as usize][Black as usize] = sides << Direction::South;
        }

        // Then use those to generate kings
        unsafe {
            ATT_KING[square as usize] = ATT_PAWNS[square as usize][White as usize]
                | ATT_PAWNS[square as usize][Black as usize]
                | sides
                | (s << Direction::North)
                | (s << Direction::South);
        }

        // Now, knight moves
        for dir in [Direction::North, Direction::South] {
            let dde = s << dir << dir << Direction::East;
            let ddw = s << dir << dir << Direction::West;
            let dee = (s << dir << Direction::East) << Direction::East;
            let dww = (s << dir << Direction::West) << Direction::West;

            unsafe {
                ATT_KNIGHT[square as usize] |= dde | ddw | dee | dww;
            }
        }
    }

    IS_INIT.set(true).unwrap();
}

// TODO Maybe store in a module not named `precompute`?
pub(crate) fn ray(square: Square, dir: Direction) -> Bitboard {
    unsafe { BB_RAYS[square as usize][dir as usize] }
}
pub(crate) fn line(a: Square, b: Square) -> Bitboard {
    unsafe { BB_LINES[a as usize][b as usize] }
}

pub(crate) fn pawn_attacks(square: Square, color: Color) -> Bitboard {
    unsafe { ATT_PAWNS[square as usize][color as usize] }
}
pub(crate) fn knight_attacks(square: Square) -> Bitboard {
    unsafe { ATT_KNIGHT[square as usize] }
}
pub(crate) fn king_attacks(square: Square) -> Bitboard {
    unsafe { ATT_KING[square as usize] }
}

pub(crate) fn bishop_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let mut rv = 0.into();

    for dir in Direction::diagonal() {
        let att = ray(square, dir);
        let possible = att & occupancy;
        if bool::from(possible) {
            let blocker = if dir.is_forward() {
                possible.lsb()
            } else {
                possible.msb()
            };
            rv |= att ^ ray(blocker, dir);
        } else {
            rv |= att;
        }
    }

    rv
}
pub(crate) fn rook_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let mut rv = 0.into();

    for dir in Direction::orthogonal() {
        let att = ray(square, dir);
        let possible = att & occupancy;
        if bool::from(possible) {
            let blocker = if dir.is_forward() {
                possible.lsb()
            } else {
                possible.msb()
            };
            rv |= att ^ ray(blocker, dir);
        } else {
            rv |= att;
        }
    }

    rv
}
pub(crate) fn queen_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    let mut rv = 0.into();

    for dir in Direction::all() {
        let att = ray(square, dir);
        let possible = att & occupancy;
        if bool::from(possible) {
            let blocker = if dir.is_forward() {
                possible.lsb()
            } else {
                possible.msb()
            };
            rv |= att ^ ray(blocker, dir);
        } else {
            rv |= att;
        }
    }

    rv
}
