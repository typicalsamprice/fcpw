use std::hint::assert_unchecked;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Neg, Not};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::precompute;
use crate::square::{Direction, File, Rank, Square};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
    pub const fn into_inner(self) -> u64 {
        self.0
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
    pub unsafe fn msb_unchecked(self) -> Square {
        assert_unchecked(self.0 != 0);
        std::mem::transmute(63 - self.0.trailing_zeros() as u8)
    }

    pub fn has(self, sq: Square) -> bool {
        (self & Self::from(sq)).0 > 0
    }

    pub const fn zero(self) -> bool {
        self.0 == 0
    }
    pub const fn nonzero(self) -> bool {
        !self.zero()
    }
    pub fn more_than_one(self) -> bool {
        self.0 & (self.0.wrapping_sub(1)) > 0
    }

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

    pub const fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    pub const fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
    pub const fn bitxor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }
    pub const fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
    pub const fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
    }
    pub const fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    pub const fn not(self) -> Self {
        Self(!self.0)
    }
    pub const fn neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }

    pub const fn from_rank(rank: Rank) -> Self {
        Self(0xff_u64 << (rank as usize * 8))
    }
    pub const fn from_file(file: File) -> Self {
        let bb = 0x0101010101010101u64;
        Self(bb << (file as usize))
    }
    pub const fn from_square(square: Square) -> Self {
        Self(1u64 << (square as usize))
    }

    pub const fn from_ranks<const N: usize>(ranks: [Rank; N]) -> Self {
        let mut rv = Self(0);
        let mut i = 0;
        while i < N {
            rv.bitor_assign(Self::from_rank(ranks[i]));
            i += 1;
        }
        rv
    }
    pub const fn from_files<const N: usize>(files: [File; N]) -> Self {
        let mut rv = Self(0);
        let mut i = 0;
        while i < N {
            rv.bitor_assign(Self::from_file(files[i]));
            i += 1;
        }
        rv
    }
    pub const fn from_squares<const N: usize>(squares: [Square; N]) -> Self {
        let mut rv = Self(0);
        let mut i = 0;
        while i < N {
            rv.bitor_assign(Self::from_square(squares[i]));
            i += 1;
        }
        rv
    }

    pub const fn shl(self, shift: i32) -> Self {
        Self(self.0 << shift)
    }
    pub const fn shr(self, shift: i32) -> Self {
        Self(self.0 >> shift)
    }

    pub const fn shift(self, dir: Direction) -> Self {
        use Direction::*;
        match dir {
            East => self.shl(1).bitand(Self::from_file(File::A).not()),
            West => self.shr(1).bitand(Self::from_file(File::H).not()),
            North => self.shl(8),
            South => self.shr(8),
            NorthEast => self.shift(North).shift(East),
            NorthWest => self.shift(North).shift(West),
            SouthEast => self.shift(South).shift(East),
            SouthWest => self.shift(South).shift(West),
        }
    }

    pub const fn sub(self, other: Self) -> Self {
        Self(self.0.wrapping_sub(other.0))
    }
    pub const fn mul(self, other: Self) -> Self {
        Self(self.0.wrapping_mul(other.0))
    }
    pub const fn add(self, other: Self) -> Self {
        Self(self.0.wrapping_add(other.0))
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
        Self::from_square(value)
    }
}
impl From<Option<Square>> for Bitboard {
    fn from(value: Option<Square>) -> Self {
        match value {
            Some(s) => Self::from_square(s),
            None => Self::new(0),
        }
    }
}
impl From<File> for Bitboard {
    fn from(value: File) -> Self {
        Self::from_file(value)
    }
}
impl From<Rank> for Bitboard {
    fn from(value: Rank) -> Self {
        Self::from_rank(value)
    }
}
impl<T> From<&[T]> for Bitboard
where
    T: Into<Bitboard> + Copy,
{
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
            let s = unsafe { self.0.lsb_unchecked() };
            self.0 &= self.0.sub(Bitboard::new(1));
            Some(s)
        }
    }
}
impl DoubleEndedIterator for BitboardIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if u64::from(self.0) == 0 {
            None
        } else {
            let s = unsafe { self.0.msb_unchecked() };
            self.0 ^= Bitboard::from_square(s);
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
        self.not()
    }
}
impl Neg for Bitboard {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.neg()
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.bitand(rhs)
    }
}
impl BitAnd<&Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: &Bitboard) -> Self::Output {
        self.bitand(*rhs)
    }
}
impl BitAnd for &Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: &Bitboard) -> Self::Output {
        (*self).bitand(*rhs)
    }
}
impl BitAnd<Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        (*self).bitand(rhs)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.bitor(rhs)
    }
}
impl BitOr<&Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: &Bitboard) -> Self::Output {
        self.bitor(*rhs)
    }
}
impl BitOr for &Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: &Bitboard) -> Self::Output {
        (*self).bitor(*rhs)
    }
}
impl BitOr<Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        (*self).bitor(rhs)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bitxor(rhs)
    }
}
impl BitXor<&Bitboard> for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: &Bitboard) -> Self::Output {
        self.bitxor(*rhs)
    }
}
impl BitXor for &Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: &Bitboard) -> Self::Output {
        (*self).bitxor(*rhs)
    }
}
impl BitXor<Bitboard> for &Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        (*self).bitxor(rhs)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.bitand_assign(rhs);
    }
}
impl BitAndAssign<&Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: &Bitboard) {
        self.bitand_assign(*rhs);
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bitor_assign(rhs);
    }
}
impl BitOrAssign<&Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: &Bitboard) {
        self.bitor_assign(*rhs);
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bitxor_assign(rhs);
    }
}
impl BitXorAssign<&Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, rhs: &Bitboard) {
        self.bitxor_assign(*rhs);
    }
}

impl Shl<i32> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: i32) -> Self::Output {
        self.shl(rhs)
    }
}
impl Shr<i32> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: i32) -> Self::Output {
        self.shr(rhs)
    }
}

impl ShlAssign<i32> for Bitboard {
    fn shl_assign(&mut self, rhs: i32) {
        *self = self.shl(rhs);
    }
}
impl ShrAssign<i32> for Bitboard {
    fn shr_assign(&mut self, rhs: i32) {
        *self = self.shr(rhs);
    }
}

impl Shl<Direction> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: Direction) -> Self::Output {
        self.shift(rhs)
    }
}
impl ShlAssign<Direction> for Bitboard {
    fn shl_assign(&mut self, rhs: Direction) {
        *self = self.shift(rhs);
    }
}
