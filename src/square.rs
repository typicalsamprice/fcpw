use std::mem::transmute;
use std::ops::Not;

use crate::bitboard::Bitboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Square {
    #[inline]
    pub const fn new(file: File, rank: Rank) -> Self {
        let sq_idx = ((rank as u8) << 3) + (file as u8);
        // SAFETY: Bounds of file/rank enums make this bounded propertly in [0, 63].
        unsafe { transmute(sq_idx) }
    }

    #[inline]
    pub const fn file(self) -> File {
        // SAFETY: Limits of square enum makes this bounded properly.
        unsafe { transmute(self as u8 & 7) }
    }
    #[inline]
    pub const fn rank(self) -> Rank {
        // SAFETY: Limits of square enum makes this bounded properly.
        unsafe { transmute(self as u8 >> 3) }
    }

    #[inline]
    pub fn distance(self, other: Square) -> i32 {
        let rank_dist = (self.rank() as u8).abs_diff(other.rank() as u8);
        let file_dist = (self.file() as u8).abs_diff(other.file() as u8);
        rank_dist.max(file_dist) as i32
    }

    pub fn dir_to(self, other: Square) -> Option<Direction> {
        if !self.same_line(other) {
            return None;
        }

        if self.rank() == other.rank() {
            if self > other {
                return Some(Direction::West);
            } else {
                return Some(Direction::East);
            }
        } else if self.file() == other.file() {
            if self > other {
                return Some(Direction::South);
            } else {
                return Some(Direction::North);
            }
        }

        Some(
            match (self.rank() > other.rank(), self.file() > other.file()) {
                (true, true) => Direction::SouthWest,
                (true, false) => Direction::SouthEast,
                (false, true) => Direction::NorthWest,
                (false, false) => Direction::NorthEast,
            },
        )
    }

    pub fn same_line(self, other: Square) -> bool {
        if self == other {
            return false; // Unhelpful to say true.
        }

        if self.rank() == other.rank() || self.file() == other.file() {
            return true;
        }

        let file_diff = (self.file() as u8).abs_diff(other.file() as u8);
        let rank_diff = (self.rank() as u8).abs_diff(other.rank() as u8);

        file_diff == rank_diff
    }

    #[inline]
    pub fn shift(self, dir: Direction) -> Option<Self> {
        (Bitboard::from(self) << dir).into_iter().next()
    }
    #[inline]
    pub unsafe fn shift_unchecked(self, dir: Direction) -> Self {
        self.shift(dir).unwrap_unchecked()
    }
}

impl From<Square> for u8 {
    fn from(value: Square) -> Self {
        value as Self
    }
}

impl TryFrom<[u8; 2]> for Square {
    type Error = ();
    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        if value[0] < b'a' || value[1] < b'0' {
            return Err(());
        }

        let f = value[0] - b'a';
        let r = value[1] - b'0';

        if f >= 8 || r >= 8 {
            return Err(());
        }

        // SAFETY: Bounds checked above.
        let file = unsafe { std::mem::transmute(f) };
        let rank = unsafe { std::mem::transmute(r) };
        Ok(Self::new(file, rank))
    }
}
impl TryFrom<&[u8]> for Square {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            Err(())
        } else {
            Self::try_from([value[0], value[1]])
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
}

impl Direction {
    #[inline]
    pub const fn all() -> [Self; 8] {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::NorthWest,
            Direction::SouthWest,
        ]
    }
    #[inline]
    pub const fn orthogonal() -> [Self; 4] {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }
    #[inline]
    pub const fn diagonal() -> [Self; 4] {
        [
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::NorthWest,
            Direction::SouthWest,
        ]
    }

    #[inline]
    pub const fn is_forward(self) -> bool {
        use Direction::*;
        match self {
            North | NorthEast | NorthWest | East => true,
            _ => false,
        }
    }
}

impl Not for Direction {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        use Direction::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
            NorthEast => SouthWest,
            SouthWest => NorthEast,
            NorthWest => SouthEast,
            SouthEast => NorthWest,
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl From<Rank> for char {
    fn from(value: Rank) -> Self {
        (b'1' + value as u8) as char
    }
}
impl From<File> for char {
    fn from(value: File) -> Self {
        (b'a' + value as u8) as char
    }
}

impl TryFrom<u8> for Rank {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..8 => Ok(unsafe { std::mem::transmute(value) }),
            8.. => Err(()),
        }
    }
}
impl TryFrom<u8> for File {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..8 => Ok(unsafe { std::mem::transmute(value) }),
            8.. => Err(()),
        }
    }
}
