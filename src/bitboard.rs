use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Neg, Not};

use crate::square::{File, Rank, Square};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn lsb(self) -> Square {
        assert_ne!(self.0, 0);
        let index = self.0.trailing_zeros() as u8;
        // SAFETY: This index is less than 64, since the internal u64 is nonzero.
        unsafe { std::mem::transmute(index) }
    }
    pub fn has(self, sq: Square) -> bool {
        (self & Self::from(sq)).0 > 0
    }

    pub fn more_than_one(self) -> bool {
        self.0 & (self.0.wrapping_sub(1)) > 0
    }

    pub fn popcount(self) -> i32 {
        self.0.count_ones() as i32
    }

    pub fn interval(a: Square, b: Square) -> Self {
        // TODO Cache this
        let mut rv = 0.into();

        if !a.same_line(b) || a == b {
            return rv;
        }

        if a > b {
            return Bitboard::interval(b, a);
        }

        let mut cur_u = a as u8;
        let b_u8 = b as u8;

        // We have to find the increment we'll use.
        let inc = if a.rank() == b.rank() {
            8
        } else if a.file() == b.file() {
            1
        } else if a.file() < b.file() {
            9
        } else {
            7
        };

        while cur_u + inc < b_u8 {
            cur_u += inc;
            // Safety: this is less than 64, since it is less than b_u8 < 64.
            let s: Square = unsafe { std::mem::transmute(cur_u) };
            rv |= Bitboard::from(s);
        }

        rv
    }
}

#[derive(Debug)]
pub struct BitboardIter(Bitboard);

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bb_str = String::new();

        for fake_rank_index in 0..8 {
            let rank_index = 7 - fake_rank_index;
            for file_index in 0..8 {
                let sq_index: u8 = (file_index << 3) + rank_index;
                let s: Square = unsafe { std::mem::transmute(sq_index) };
                if self.has(s) {
                    bb_str += "X";
                } else {
                    bb_str += ".";
                }
                if file_index != 7 {
                    bb_str += " ";
                }
            }
            bb_str += "\n";
        }

        write!(f, "{bb_str}")
    }
}

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}
impl From<Bitboard> for u64 {
    fn from(value: Bitboard) -> Self {
        value.0
    }
}

impl From<Bitboard> for bool {
    fn from(value: Bitboard) -> Self {
        value.0 != 0
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Self(1u64 << (value as u8))
    }
}
impl From<File> for Bitboard {
    fn from(value: File) -> Self {
        let bb = 0x0101010101010101u64;
        Self(bb << (value as u8))
    }
}
impl From<Rank> for Bitboard {
    fn from(value: Rank) -> Self {
        let shift = (value as u8) * 8;
        Self(0xffu64 << shift)
    }
}
impl From<&[Square]> for Bitboard {
    fn from(squares: &[Square]) -> Self {
        let mut rv = Self::from(0);
        for &sq in squares {
            rv |= Self::from(sq);
        }
        rv
    }
}
impl<const N: usize> From<[Square; N]> for Bitboard {
    fn from(squares: [Square; N]) -> Self {
        Self::from(&squares[..])
    }
}

impl Iterator for BitboardIter {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        if u64::from(self.0) == 0 {
            None
        } else {
            let s = self.0.lsb();
            self.0 ^= Bitboard::from(s);
            Some(s)
        }
    }
}
impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;
    fn into_iter(self) -> Self::IntoIter {
        BitboardIter(self)
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}
impl Neg for Bitboard {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(self.0.wrapping_neg())
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}
impl BitAnd<&Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}
impl BitAnd for &Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}
impl BitAnd<Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}
impl BitOr<&Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}
impl BitOr for &Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}
impl BitOr<Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}
impl BitXor<&Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}
impl BitXor for &Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}
impl BitXor<Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl BitAndAssign<&Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: &Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl BitOrAssign<&Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: &Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
impl BitXorAssign<&Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, rhs: &Bitboard) {
        self.0 ^= rhs.0;
    }
}
