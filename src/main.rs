mod bitboard;
mod color;
mod movegen;
mod piece;
mod position;
mod square;

use bitboard::Bitboard;
use square::Square::*;

fn main() {
    let s = Bitboard::from([A1, B3, C5, F2, H1, H8]);
    let a1h8 = Bitboard::interval(H8, A1);
    println!("{s}");
    println!("{a1h8}");
    for sq in s {
        println!("{sq:?}");
    }
}
