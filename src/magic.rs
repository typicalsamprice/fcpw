#[cfg(feature = "pext")]
use std::arch::x86_64::_pext_u64;
//use bitintr::Pext;

#[cfg(feature = "pext")]
fn pext(a: u64, b: u64) -> u64 {
    unsafe { _pext_u64(a, b) }
}

use crate::piece::PieceType::{self, Bishop, Rook};
use crate::{Bitboard, Direction, File, Rank, Square};

#[derive(Debug, Clone, Copy)]
struct Magic {
    pointer: *const Bitboard,
    mask: Bitboard,
    magic: Bitboard,
    #[cfg(not(feature = "pext"))]
    shift: i32,
}

// Trust me bro.
// In reality, we alter it here, but that const pointer truly will not be changed once initialized.
// This means no dangling/nullity (also, it's for a static setup)

#[derive(Debug, Clone, Copy)]
struct SeededPRNG(u64);

// https://vigna.di.unimi.it/ftp/papers/xorshift.pdf
impl SeededPRNG {
    fn get(&mut self) -> u64 {
        assert_ne!(self.0, 0);
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;

        self.0.wrapping_mul(2685821657736338717)
    }

    fn roll(&mut self) -> u64 {
        self.get() & self.get() & self.get()
    }
}

static mut BISHOP_MAGICS: [Magic; 64] = [Magic::new(); 64];
static mut ROOK_MAGICS: [Magic; 64] = [Magic::new(); 64];

static mut BISHOP_ATTACKS: [Bitboard; 0x1480] = [Bitboard::new(0); 0x1480];
static mut ROOK_ATTACKS: [Bitboard; 0x19000] = [Bitboard::new(0); 0x19000];

impl Magic {
    const fn new() -> Self {
        Self {
            pointer: std::ptr::null(),
            mask: Bitboard::new(0),
            magic: Bitboard::new(0),
            #[cfg(not(feature = "pext"))]
            shift: 0,
        }
    }

    #[cfg(feature = "pext")]
    fn index(&self, occupancy: Bitboard) -> isize {
        pext(u64::from(occupancy), u64::from(self.mask)) as isize
    }

    #[cfg(not(feature = "pext"))]
    fn index(&self, occupancy: Bitboard) -> isize {
        ((self.mask & occupancy).mul(self.magic) >> self.shift).into_inner() as isize
    }

    fn attack(&self, occupancy: Bitboard) -> Bitboard {
        assert!(!self.pointer.is_null());
        unsafe { *self.pointer.offset(self.index(occupancy)) }
    }
}

pub(crate) fn bishop_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    unsafe { BISHOP_MAGICS[square as usize] }.attack(occupancy)
}
pub(crate) fn rook_attacks(square: Square, occupancy: Bitboard) -> Bitboard {
    unsafe { ROOK_MAGICS[square as usize] }.attack(occupancy)
}

const fn slider_gen(square: Square, occ: Bitboard, is_rook: bool) -> Bitboard {
    let dirs = if is_rook {
        Direction::orthogonal()
    } else {
        Direction::diagonal()
    };

    let mut rv = Bitboard::new(0);
    let mask = occ.not();

    let mut i = 0;
    while i < 4 {
        let mut s = Bitboard::from_square(square);
        let dir = dirs[i];
        while s.nonzero() {
            s = s.shift(dir);

            rv.bitor_assign(s);

            s.bitand_assign(mask);
        }

        i += 1;
    }

    rv
}

fn init_magics_for(magic_table: *mut Magic, table: *mut Bitboard, is_rook: bool) {
    #[cfg(not(feature = "pext"))]
    let seeds = [728, 10316, 55013, 32803, 12281, 15100, 16645, 255];
    #[cfg(not(feature = "pext"))]
    let mut occupancy = [Bitboard::new(0); 4096];
    #[cfg(not(feature = "pext"))]
    let mut epoch = [0; 4096];
    #[cfg(not(feature = "pext"))]
    let mut count = 0;

    let mut reference = [Bitboard::new(0); 4096];
    let mut size = 0;

    for square in Bitboard::new(0).not() {
        let edges = (Bitboard::from([Rank::One, Rank::Eight]) & !Bitboard::from(square.rank()))
            | (Bitboard::from([File::A, File::H]) & !Bitboard::from(square.file()));
        let m = unsafe { &mut *magic_table.offset(square as isize) };
        m.mask = slider_gen(square, 0.into(), is_rook) & !edges;

        #[cfg(not(feature = "pext"))]
        {
            m.shift = 64 - m.mask.popcount();
        }

        m.pointer = if square == Square::A1 {
            table as *const Bitboard
        } else {
            let ptr = unsafe { &*magic_table.offset((square as isize) - 1) }.pointer;
            unsafe { ptr.offset(size as isize) }
        };
        size = 0;

        let mut b: Bitboard = 0.into();
        loop {
            #[cfg(not(feature = "pext"))]
            {
                occupancy[size] = b;
            }

            reference[size] = slider_gen(square, b, is_rook);

            #[cfg(feature = "pext")]
            unsafe {
                let pxt = pext(b.into_inner(), m.mask.into_inner());
                *(m.pointer.offset(pxt as isize) as *mut _) = reference[size];
            }

            size += 1;
            b = (b.sub(m.mask)) & m.mask;

            if b.zero() {
                break;
            }
        }

        #[cfg(not(feature = "pext"))]
        {
            let mut prng = SeededPRNG(seeds[square.rank() as usize]);
            let mut i = 0;

            while i < size {
                m.magic = 0.into();
                while (m.magic.mul(m.mask) >> 56).popcount() < 6 {
                    m.magic = Bitboard::new(prng.roll());
                }

                count += 1;
                i = 0;
                while i < size {
                    let index = m.index(occupancy[i]);

                    if epoch[index as usize] < count {
                        epoch[index as usize] = count;
                        unsafe {
                            *(m.pointer.offset(index) as *mut _) = reference[i];
                        }
                    } else if unsafe { *m.pointer.offset(index) } != reference[i] {
                        break;
                    }

                    i += 1;
                }
            }
        }
    }
}

pub(crate) fn init_magics() {
    init_magics_for(
        &raw mut BISHOP_MAGICS as *mut Magic,
        &raw mut BISHOP_ATTACKS as *mut Bitboard,
        false,
    );
    init_magics_for(
        &raw mut ROOK_MAGICS as *mut Magic,
        &raw mut ROOK_ATTACKS as *mut Bitboard,
        true,
    );
}
