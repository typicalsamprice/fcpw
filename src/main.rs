#![allow(dead_code, unused_imports)]
mod bitboard;
mod color;
mod macros;
#[cfg(feature = "magic")]
mod magic;
mod movegen;
mod perft;
mod piece;
mod position;
mod precompute;
mod square;

use position::Position;

fn main() {
    precompute::initialize();
    let pos = Position::default();
    println!("{pos}");
}
