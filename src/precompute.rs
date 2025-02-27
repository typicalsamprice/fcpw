#[cfg(feature = "magic")]
use crate::magic;
use std::sync::OnceLock;

// TODO Precompute elements
// - Piece moves, including sliding pieces (start with rays for simplicity, transition to magic bitboards if required)
use crate::bitboard::Bitboard;
use crate::color::Color::{self, *};
use crate::square::{Direction, Square};

static IS_INIT: OnceLock<bool> = OnceLock::new();

static mut BB_RAYS: [[Bitboard; 8]; 64] = [[Bitboard::EMPTY; 8]; 64];
static mut BB_LINES: [[Bitboard; 64]; 64] = [[Bitboard::EMPTY; 64]; 64];

static mut ATT_KNIGHT: [Bitboard; 64] = [Bitboard::EMPTY; 64];
static mut ATT_KING: [Bitboard; 64] = [Bitboard::EMPTY; 64];
static mut ATT_PAWNS: [[Bitboard; 2]; 64] = [[Bitboard::EMPTY; 2]; 64];

pub fn initialize() {
    if IS_INIT.get() == Some(&true) {
        return;
    }

    #[cfg(feature = "magic")]
    magic::init_magics();

    // Setup for ray/line caching
    for square in Bitboard::FULL {
        for d in Direction::all() {
            let mut s = Bitboard::from(square);
            let mut r = Bitboard::EMPTY;
            while bool::from(s) {
                s <<= d;
                r |= s;
            }
            unsafe { BB_RAYS[square as usize][d as usize] = r };
        }

        for other in Bitboard::FULL {
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
    for square in Bitboard::FULL {
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
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn ray(square: Square, dir: Direction) -> Bitboard {
    unsafe { BB_RAYS[square as usize][dir as usize] }
}
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn line(a: Square, b: Square) -> Bitboard {
    unsafe { BB_LINES[a as usize][b as usize] }
}

#[cfg_attr(feature = "inline", inline)]
pub(crate) fn pawn_attacks(square: Square, color: Color) -> Bitboard {
    unsafe { ATT_PAWNS[square as usize][color as usize] }
}
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn knight_attacks(square: Square) -> Bitboard {
    unsafe { ATT_KNIGHT[square as usize] }
}
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn king_attacks(square: Square) -> Bitboard {
    unsafe { ATT_KING[square as usize] }
}

#[cfg(not(feature = "magic"))]
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn bishop_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    sliders(square, occupancy, &Direction::diagonal())
}
#[cfg(not(feature = "magic"))]
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn rook_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    sliders(square, occupancy, &Direction::orthogonal())
}
#[cfg(not(feature = "magic"))]
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn queen_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    sliders(square, occupancy, &Direction::all())
}

#[cfg(not(feature = "magic"))]
fn sliders(square: Square, occupancy: Bitboard, dirs: &[Direction]) -> Bitboard {
    let mut rv = Bitboard::EMPTY;

    for &dir in dirs {
        let attacks = ray(square, dir);
        let blockers = attacks & occupancy;
        if bool::from(blockers) {
            let blocker = if dir.is_forward() {
                blockers.lsb()
            } else {
                blockers.msb()
            };
            let up_to_blocker = attacks ^ ray(blocker, dir);
            rv |= up_to_blocker;
        } else {
            rv |= attacks;
        }
    }

    rv
}

#[cfg(feature = "magic")]
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn bishop_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    magic::bishop_attacks(square, occupancy)
}
#[cfg(feature = "magic")]
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn rook_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    magic::rook_attacks(square, occupancy)
}
#[cfg(feature = "magic")]
#[cfg_attr(feature = "inline", inline)]
pub(crate) fn queen_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    magic::bishop_attacks(square, occupancy) | magic::rook_attacks(square, occupancy)
}
