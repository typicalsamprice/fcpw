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
    #[cfg(feature = "magic")]
    {
        init_magics();
        println!("init_magics called.");
    }

    let mut pos = Position::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -");
    pos.make_move(Move::new(B4, B1));
    pos.make_move(Move::new(H4, G5));
    pos.make_move(Move::new(A5, B4));
    pos.make_move(Move::new(C7, C5));
    println!("{pos}");
    println!(
        "Ray-based Bishop on E3:\n{}",
        precompute::bishop_attacks(E3, pos.all())
    );
    #[cfg(feature = "magic")]
    println!(
        "Magic-based Bishop on E3:\n{}",
        magic::bishop_attacks(E3, pos.all())
    );
}
