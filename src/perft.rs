use crate::movegen::generate;
use crate::position::Position;

pub fn perft(pos: &mut Position, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let leaf = depth == 1;
    let mut nodes = 0;
    let moves = generate::legal(pos);

    if depth == 1 {
        return moves.len();
    }

    for x in moves {
        pos.make_move(x);
        let c = perft__(pos, depth - 1);
        nodes += c;
        println!("{x}: {c}");
        pos.unmake_move(x);
    }

    nodes
}

fn perft__(pos: &mut Position, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let leaf = depth == 1;
    let mut nodes = 0;
    let moves = generate::legal(pos);

    if depth == 1 {
        return moves.len();
    }

    for x in moves {
        pos.make_move(x);
        nodes += perft__(pos, depth - 1);
        pos.unmake_move(x);
    }

    nodes
}

#[cfg(test)]
#[ctor::ctor]
fn test_inits() {
    use crate::precompute;

    precompute::initialize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startpos_depth_1() {
        let mut pos = Position::default();
        assert_eq!(perft(&mut pos, 1), 20);
    }
    #[test]
    fn startpos_depth_2() {
        let mut pos = Position::default();
        assert_eq!(perft(&mut pos, 2), 400);
    }
    #[test]
    fn startpos_depth_3() {
        let mut pos = Position::default();
        assert_eq!(perft(&mut pos, 3), 8902);
    }
    #[test]
    fn startpos_depth_4() {
        let mut pos = Position::default();
        assert_eq!(perft(&mut pos, 4), 197281);
    }
    #[test]
    fn startpos_depth_5() {
        let mut pos = Position::default();
        assert_eq!(perft(&mut pos, 5), 4865609);
    }
    #[test]
    fn startpos_depth_6() {
        let mut pos = Position::default();
        assert_eq!(perft(&mut pos, 6), 119060324);
    }

    #[test]
    fn kiwipete_depth_1() {
        let mut pos = Position::new_from_fen(Position::KIWIPETE_FEN);
        assert_eq!(perft(&mut pos, 1), 48);
    }
    #[test]
    fn kiwipete_depth_2() {
        let mut pos = Position::new_from_fen(Position::KIWIPETE_FEN);
        assert_eq!(perft(&mut pos, 2), 2039);
    }
    #[test]
    fn kiwipete_depth_3() {
        let mut pos = Position::new_from_fen(Position::KIWIPETE_FEN);
        assert_eq!(perft(&mut pos, 3), 97862);
    }
    #[test]
    fn kiwipete_depth_4() {
        let mut pos = Position::new_from_fen(Position::KIWIPETE_FEN);
        assert_eq!(perft(&mut pos, 4), 4085603);
    }

    static POS3_FEN: &'static str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -";

    #[test]
    fn pos3_depth_1() {
        let mut pos = Position::new_from_fen(POS3_FEN);
        assert_eq!(perft(&mut pos, 1), 14);
    }
    #[test]
    fn pos3_depth_2() {
        let mut pos = Position::new_from_fen(POS3_FEN);
        assert_eq!(perft(&mut pos, 2), 191);
    }
    #[test]
    fn pos3_depth_3() {
        let mut pos = Position::new_from_fen(POS3_FEN);
        assert_eq!(perft(&mut pos, 3), 2812);
    }
    #[test]
    fn pos3_depth_4() {
        let mut pos = Position::new_from_fen(POS3_FEN);
        assert_eq!(perft(&mut pos, 4), 43238);
    }
    #[test]
    fn pos3_depth_5() {
        let mut pos = Position::new_from_fen(POS3_FEN);
        assert_eq!(perft(&mut pos, 5), 674624);
    }
}
