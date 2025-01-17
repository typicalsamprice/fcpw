use crate::movegen::generate;
use crate::position::Position;

pub fn perft(pos: &mut Position, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

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
    macro_rules! create_suite {
        ($name:ident, $fen:expr, $results:expr) => {
            mod $name {
                const RES: [usize; 5] = $results;
                const FEN: &str = $fen;

                use super::super::{perft, Position};

                #[test]
                fn depth_1() {
                    let mut pos = Position::new_from_fen(FEN);
                    assert_eq!(perft(&mut pos, 1), RES[0]);
                }
                #[test]
                fn depth_2() {
                    let mut pos = Position::new_from_fen(FEN);
                    assert_eq!(perft(&mut pos, 2), RES[1]);
                }
                #[test]
                fn depth_3() {
                    let mut pos = Position::new_from_fen(FEN);
                    assert_eq!(perft(&mut pos, 3), RES[2]);
                }
                #[test]
                fn depth_4() {
                    let mut pos = Position::new_from_fen(FEN);
                    if RES[3] > 0 {
                        assert_eq!(perft(&mut pos, 4), RES[3]);
                    }
                }
                #[test]
                #[ignore = "depth 5 generally takes too long"]
                fn depth_5() {
                    let mut pos = Position::new_from_fen(FEN);
                    if RES[4] > 0 {
                        assert_eq!(perft(&mut pos, 5), RES[4]);
                    }
                }
            }
        };
    }

    create_suite!(
        startpos,
        Position::STARTING_FEN,
        [20, 400, 8902, 197281, 4865609]
    );

    create_suite!(
        kiwipete,
        Position::KIWIPETE_FEN,
        [48, 2039, 97862, 4085603, 193690690]
    );

    create_suite!(
        cpw_pos_3,
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -",
        [14, 191, 2812, 43238, 674624]
    );

    create_suite!(
        cpw_pos_4,
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        [6, 264, 9467, 422333, 15833292]
    );

    create_suite!(
        cpw_pos_5,
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        [44, 1486, 62379, 2103487, 89941194]
    );

    // Taken from https://lichess.org/nD3qQlh0#29
    create_suite!(
        my_lichess_1,
        "r6r/pp1k2pp/3bp1q1/2p2nB1/3p2Q1/1N1P3P/PPP2PP1/R3R1K1 b - - 0 15",
        [43, 1916, 77347, 3296388, 129476614]
    );

    // Taken from https://lichess.org/oXy9Eebe/black#42
    create_suite!(
        my_lichess_2,
        "3r1rk1/1p2b1p1/n2pp1np/4p3/1P2P3/2q1NNB1/Q4PPP/R2R2K1 w - - 0 22",
        [44, 1935, 81291, 3515320, 146996597]
    );
}
