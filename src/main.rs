mod bitboard;
mod color;
mod movegen;
mod piece;
mod position;
mod precompute;
mod square;

use bitboard::Bitboard;
use square::*;
use Direction::*;
use Square::*;

fn main() {
    precompute::initialize();
    let x = Bitboard::interval(A1, H8);
    let y = precompute::line(A1, G8);
    println!("{x}");
    println!("{y}");
}
