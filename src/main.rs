#![allow(unused)]
mod bitboard;
mod color;
mod macros;
mod movegen;
mod piece;
mod position;
mod precompute;
mod square;

use bitboard::Bitboard;
use color::Color::{self, *};
use position::Position;
use square::*;
use Direction::*;
use Square::*;

fn main() {
    precompute::initialize();
    let pos = Position::new_from_fen(Position::STARTING_FEN);
    println!("{pos}");
}
