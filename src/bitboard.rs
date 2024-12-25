use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Bitboard(u64);

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(self.0.not())
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
