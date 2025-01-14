#![allow(unused)]
mod bitboard;
mod color;
mod macros;
mod movegen;
mod piece;
mod position;
mod precompute;
mod square;

use crate::movegen::Move;
use bitboard::Bitboard;
use color::Color::{self, *};
use position::Position;
use square::*;
use Direction::*;
use Square::*;

fn main() {
    precompute::initialize();
    let mut pos = Position::default();
    pos.make_move(Move::new(E2, E4));
    println!("{pos}");
}
