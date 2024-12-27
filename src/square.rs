use std::mem::transmute;

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
    pub const fn new(file: File, rank: Rank) -> Self {
        let sq_idx = ((file as u8) << 3) + (rank as u8);
        // SAFETY: Bounds of file/rank enums make this bounded propertly in [0, 63].
        unsafe { transmute(sq_idx) }
    }

    pub const fn file(self) -> File {
        // SAFETY: Limits of square enum makes this bounded properly.
        unsafe { transmute(self as u8 >> 3) }
    }
    pub const fn rank(self) -> Rank {
        // SAFETY: Limits of square enum makes this bounded properly.
        unsafe { transmute(self as u8 & 7) }
    }

    pub fn distance(self, other: Square) -> i32 {
        let rank_dist = (self.rank() as u8).abs_diff(other.rank() as u8);
        let file_dist = (self.file() as u8).abs_diff(other.file() as u8);
        rank_dist.max(file_dist) as i32
    }

    pub fn same_line(self, other: Square) -> bool {
        if self.rank() == other.rank() || self.file() == other.file() {
            return true;
        }

        let file_diff = (self.file() as u8).abs_diff(other.file() as u8);
        let rank_diff = (self.rank() as u8).abs_diff(other.rank() as u8);

        file_diff == rank_diff
    }
}

impl From<Square> for u8 {
    fn from(value: Square) -> Self {
        value as Self
    }
}
