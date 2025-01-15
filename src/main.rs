#![allow(unused)]
mod bitboard;
mod color;
mod macros;
mod movegen;
mod perft;
mod piece;
mod position;
mod precompute;
mod square;

use crate::movegen::Move;
use bitboard::Bitboard;
use color::Color::{self, *};
use movegen::generate;
use perft::perft;
use position::Position;
use precompute::*;
use square::*;
use Direction::*;
use Square::*;

fn main() {
    precompute::initialize();
    let mut pos = Position::new_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -");
    pos.make_move(Move::new(B4, B1));
    pos.make_move(Move::new(H4, G5));
    pos.make_move(Move::new(A5, B4));
    pos.make_move(Move::new(C7, C5));
    println!("{pos}");
    //println!("Nodes: {}", perft(&mut pos, 2));
    for x in generate::legal(&pos) {
        println!("{x}");
    }
}
