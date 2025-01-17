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

use bitboard::Bitboard;
use color::Color::{self, *};
#[cfg(feature = "magic")]
use magic::init_magics;
use movegen::generate;
use movegen::Move;
use perft::perft;
use position::Position;
use precompute::*;
use square::*;
use Direction::*;
use Square::*;

fn main() {
    println!("Has feature `magic`: {}", cfg!(feature = "magic"));
    println!("Has feature `pext`: {}", cfg!(feature = "pext"));

    precompute::initialize();

    let mut pos = Position::default();
    pos.make_move(Move::new(D2, D3));
    pos.make_move(Move::new(A7, A6));
    for x in generate::legal(&pos) {
        println!("{x}");
    }
    println!("{}", precompute::queen_attacks(D1, 0.into()));
}
