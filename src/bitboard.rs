use std::hint::assert_unchecked;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Neg, Not};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::precompute;
use crate::square::{Direction, File, Rank, Square};

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
    pub unsafe fn lsb_unchecked(self) -> Square {
        assert_unchecked(self.0 != 0);
        std::mem::transmute(self.0.trailing_zeros() as u8)
    }
    pub fn without_lsb(self) -> Self {
        Self::new(self.0 & self.0.wrapping_sub(1))
    }

    pub fn msb(self) -> Square {
        assert_ne!(self.0, 0);
        let index = self.0.leading_zeros() as u8;
        // SAFETY: This index is less than 64, since the internal u64 is nonzero.
        unsafe { std::mem::transmute(63 - index) }
    }

    #[inline]
    pub fn has(self, sq: Square) -> bool {
        (self & Self::from(sq)).0 > 0
    }

    #[inline]
    pub fn more_than_one(self) -> bool {
        self.0 & (self.0.wrapping_sub(1)) > 0
    }

    #[inline]
    pub fn popcount(self) -> i32 {
        self.0.count_ones() as i32
    }

    pub fn interval(a: Square, b: Square) -> Self {
        if let Some(dir) = a.dir_to(b) {
            precompute::ray(a, dir) & precompute::ray(b, !dir)
        } else {
            Self::new(0)
        }
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
                let file = unsafe { File::try_from(file_index).unwrap_unchecked() };
                let rank = unsafe { Rank::try_from(rank_index).unwrap_unchecked() };
                let s = Square::new(file, rank);
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
    #[inline]
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}
impl From<Bitboard> for u64 {
    #[inline]
    fn from(value: Bitboard) -> Self {
        value.0
    }
}

impl From<Bitboard> for bool {
    #[inline]
    fn from(value: Bitboard) -> Self {
        value.0 != 0
    }
}

impl From<Square> for Bitboard {
    #[inline]
    fn from(value: Square) -> Self {
        Self(1u64 << (value as u8))
    }
}
impl From<Option<Square>> for Bitboard {
    #[inline]
    fn from(value: Option<Square>) -> Self {
        match value {
            Some(s) => Self::from(s),
            None => Self::new(0),
        }
    }
}
impl From<File> for Bitboard {
    #[inline]
    fn from(value: File) -> Self {
        let bb = 0x0101010101010101u64;
        Self(bb << (value as u8))
    }
}
impl From<Rank> for Bitboard {
    #[inline]
    fn from(value: Rank) -> Self {
        let shift = (value as u8) * 8;
        Self(0xffu64 << shift)
    }
}
impl<T> From<&[T]> for Bitboard
where
    T: Into<Bitboard> + Copy,
{
    #[inline]
    fn from(value: &[T]) -> Self {
        let mut rv = 0.into();
        for &v in value {
            rv |= v.into();
        }
        rv
    }
}

impl<const N: usize, T> From<[T; N]> for Bitboard
where
    T: Into<Bitboard>,
{
    fn from(value: [T; N]) -> Self {
        let mut rv = 0.into();

        for v in value {
            rv |= v.into();
        }

        rv
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
impl DoubleEndedIterator for BitboardIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if u64::from(self.0) == 0 {
            None
        } else {
            let s = self.0.msb();
            self.0 ^= Bitboard::from(s);
            Some(s)
        }
    }
}
impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        BitboardIter(self)
    }
}

impl Not for Bitboard {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Self(self.0.not())
    }
}
impl Neg for Bitboard {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self(self.0.wrapping_neg())
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}
impl BitAnd<&Bitboard> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitand(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}
impl BitAnd for &Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitand(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}
impl BitAnd<Bitboard> for &Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}
impl BitOr<&Bitboard> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitor(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}
impl BitOr for &Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitor(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}
impl BitOr<Bitboard> for &Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}
impl BitXor<&Bitboard> for Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitxor(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}
impl BitXor for &Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitxor(self, rhs: &Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}
impl BitXor<Bitboard> for &Bitboard {
    type Output = Bitboard;
    #[inline]
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl BitAndAssign<&Bitboard> for Bitboard {
    #[inline]
    fn bitand_assign(&mut self, rhs: &Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl BitOrAssign<&Bitboard> for Bitboard {
    #[inline]
    fn bitor_assign(&mut self, rhs: &Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
impl BitXorAssign<&Bitboard> for Bitboard {
    #[inline]
    fn bitxor_assign(&mut self, rhs: &Bitboard) {
        self.0 ^= rhs.0;
    }
}

impl Shl<i32> for Bitboard {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: i32) -> Self::Output {
        assert!(rhs < 64);
        assert!(rhs >= 0);
        Self::new(self.0.shl(rhs))
    }
}
impl Shr<i32> for Bitboard {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        assert!(rhs < 64);
        assert!(rhs >= 0);
        Self::new(self.0.shr(rhs))
    }
}

impl ShlAssign<i32> for Bitboard {
    #[inline]
    fn shl_assign(&mut self, rhs: i32) {
        *self = *self << rhs;
    }
}
impl ShrAssign<i32> for Bitboard {
    #[inline]
    fn shr_assign(&mut self, rhs: i32) {
        *self = *self >> rhs;
    }
}

impl Shl<Direction> for Bitboard {
    type Output = Self;
    #[inline]
    fn shl(self, rhs: Direction) -> Self::Output {
        use Direction::*;
        match rhs {
            North => self << 8,
            South => self >> 8,
            East => (self << 1) & !Bitboard::from(File::A),
            West => (self >> 1) & !Bitboard::from(File::H),
            NorthWest => self << North << West,
            NorthEast => self << North << East,
            SouthEast => self << South << East,
            SouthWest => self << South << West,
        }
    }
}
impl ShlAssign<Direction> for Bitboard {
    #[inline]
    fn shl_assign(&mut self, rhs: Direction) {
        *self = *self << rhs;
    }
}
