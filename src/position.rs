use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::movegen::{Move, MoveKind};
use crate::piece::{Piece, PieceType};
use crate::square::{File, Rank, Square};
use crate::{precompute, strict_cond, strict_eq, strict_ne, strict_not};

#[derive(Debug)]
pub struct Position {
    to_move: Color,
    moves: i32,

    colors: [Bitboard; 2],
    pieces: [Bitboard; 6],
    board: [Option<Piece>; 64],

    state: Option<Box<State>>,
}

#[derive(Debug)]
pub struct State {
    checkers: Bitboard,
    pinners: [Bitboard; 2],
    blockers: [Bitboard; 2],
    captured: Option<Piece>,
    en_passant: Option<Square>,

    castle_rights: u8,

    halfmoves: i32,

    previous: Option<Box<State>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleFlag {
    WhiteShort,
    WhiteLong,
    WhiteAll,
    BlackShort,
    BlackLong,
    BlackAll,
    All,
}

impl CastleFlag {
    pub const fn color(self) -> Color {
        match self {
            Self::All => panic!("CastleFlag::color called on CastleFlag::All"),
            Self::WhiteAll | Self::WhiteShort | Self::WhiteLong => Color::White,
            Self::BlackAll | Self::BlackShort | Self::BlackLong => Color::Black,
        }
    }
    pub const fn from_square(self) -> Square {
        match self.color() {
            Color::White => Square::E1,
            Color::Black => Square::E8,
        }
    }
    pub const fn to_square(self) -> Square {
        match self {
            Self::All | Self::WhiteAll | Self::BlackAll => {
                panic!("CastleFlag::to_square called on ambiguous variant.")
            }
            Self::WhiteShort => Square::G1,
            Self::WhiteLong => Square::C1,
            Self::BlackShort => Square::G8,
            Self::BlackLong => Square::C8,
        }
    }
    pub const fn rook_from_square(self) -> Square {
        match self {
            Self::All | Self::WhiteAll | Self::BlackAll => {
                panic!("CastleFlag::to_square called on ambiguous variant.")
            }
            Self::WhiteShort => Square::H1,
            Self::WhiteLong => Square::A1,
            Self::BlackShort => Square::H8,
            Self::BlackLong => Square::A8,
        }
    }
    pub const fn rook_to_square(self) -> Square {
        match self {
            Self::All | Self::WhiteAll | Self::BlackAll => {
                panic!("CastleFlag::to_square called on ambiguous variant.")
            }
            Self::WhiteShort => Square::F1,
            Self::WhiteLong => Square::D1,
            Self::BlackShort => Square::F8,
            Self::BlackLong => Square::D8,
        }
    }

    pub const fn variants_for(color: Color) -> [Self; 2] {
        match color {
            Color::White => [Self::WhiteShort, Self::WhiteLong],
            Color::Black => [Self::BlackShort, Self::BlackLong],
        }
    }
    pub const fn short_for(color: Color) -> Self {
        match color {
            Color::White => Self::WhiteShort,
            Color::Black => Self::BlackShort,
        }
    }
    pub const fn long_for(color: Color) -> Self {
        match color {
            Color::White => Self::WhiteLong,
            Color::Black => Self::BlackLong,
        }
    }
}

impl From<CastleFlag> for u8 {
    fn from(value: CastleFlag) -> Self {
        match value {
            CastleFlag::All => 0xF,
            CastleFlag::WhiteShort => 0x1,
            CastleFlag::WhiteLong => 0x2,
            CastleFlag::WhiteAll => 0x1 | 0x2,
            CastleFlag::BlackShort => 0x4,
            CastleFlag::BlackLong => 0x8,
            CastleFlag::BlackAll => 0x4 | 0x8,
        }
    }
}

impl Position {
    pub const STARTING_FEN: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn new() -> Self {
        Self {
            board: [None; 64],
            colors: [Bitboard::new(0); 2],
            moves: 0,
            pieces: [Bitboard::new(0); 6],
            to_move: Color::White,
            // SAFETY: We just created this.
            state: Some(State::new()),
        }
    }

    pub fn new_from_fen(fen: &str) -> Self {
        let mut pos = Self::new();

        let mut iter = fen.chars();

        let mut rank = Rank::Eight;
        let mut file = File::A;

        for x in iter.by_ref() {
            if x == ' ' {
                break;
            } else if x == '/' {
                strict_eq!(
                    file,
                    File::H,
                    panic!("Rank not filled in Position::new_from_fen")
                );

                assert_ne!(
                    rank,
                    Rank::One,
                    "Too many ranks in FEN given to Position::new_from_fen"
                );

                file = File::A;
                // SAFETY: We know rank != Rank::One and so (rank as u8) > 0.
                rank = unsafe { Rank::try_from(rank as u8 - 1).unwrap_unchecked() };
                continue;
            }

            if ('1'..='8').contains(&x) {
                let shiftness = x as u8 - b'0';
                let file_index = file as u8 + shiftness;

                if file_index >= 8 {
                    strict_cond!(
                        file_index <= 8,
                        panic!("File overflow in Position::new_from_fen")
                    );

                    file = File::H;
                    continue;
                }

                // SAFETY: We know file_index < 8.
                file = unsafe { File::try_from(file_index).unwrap_unchecked() };
                continue;
            }

            let Ok(p) = Piece::try_from(x) else {
                panic!("Unknown piece passed in FEN: {}", x);
            };

            let s = Square::new(file, rank);
            pos.add_piece(p, s);

            if file != File::H {
                // SAFETY: (file as u8) < 8 right now.
                file = unsafe { File::try_from(file as u8 + 1).unwrap_unchecked() };
            }
        }

        match iter.next() {
            Some('w') | Some('-') => pos.to_move = Color::White,
            Some('b') => pos.to_move = Color::Black,
            Some(x) => panic!("Position::new_from_fen: Unknown side to move in FEN: {}", x),
            None => panic!("Position::new_from_fen: FEN ended early, no side to move given."),
        }

        match iter.next() {
            Some(' ') => (),
            Some(x) => panic!("Position::new_from_fen: Unexpected character: {}", x),
            None => panic!("Position::new_from_fen: FEN ended early, no castling rights given"),
        }

        for x in iter.by_ref() {
            if x == ' ' {
                break;
            }

            if x == '-' {
                strict_eq!(pos.state().castle_rights, 0, panic!("Position::new_from_fen: Castle character '-' given with other rights given."));

                match iter.next() {
                    Some(' ') => (),
                    None | Some(_) => {
                        panic!("Position::new_from_fen: FEN ended early, no En Passant data given.")
                    }
                }
                break;
            }

            let cf = match x {
                'K' => CastleFlag::WhiteShort,
                'Q' => CastleFlag::WhiteLong,
                'k' => CastleFlag::BlackShort,
                'q' => CastleFlag::BlackLong,
                c => panic!(
                    "Position::new_from_fen: Unknown castle character given: {}",
                    c
                ),
            };

            strict_not!(
                pos.has_castle(cf),
                panic!("Position::new_from_fen: Castle flag given twice: {}", x)
            );

            pos.add_castle_right(cf);
        }

        let one = iter.next();
        let two = iter.next();

        match one {
            Some('-') => (),
            None => return pos,
            Some(f_char) => {
                let r_char = two.expect("Position::new_from_fen: en passant rank not given.");
                let f = File::try_from(f_char as u8).unwrap();
                let r = Rank::try_from(r_char as u8).unwrap();
                let s = Square::new(f, r);

                // SAFETY: Trust me bro.
                unsafe {
                    pos.state_mut().en_passant = Some(s);
                }
            }
        }

        // TODO parse move counts. not a prio.

        pos.update_state();
        pos
    }

    // Misc data pulls
    pub const fn to_move(&self) -> Color {
        self.to_move
    }
    // Bitboard pulling
    pub fn all(&self) -> Bitboard {
        self.colors[0] | self.colors[1]
    }
    pub fn color(&self, c: Color) -> Bitboard {
        self.colors[c as usize]
    }
    pub fn pieces(&self, t: PieceType) -> Bitboard {
        self.pieces[t as usize]
    }
    pub fn pieces_list(&self, ts: &[PieceType]) -> Bitboard {
        let mut res: Bitboard = 0.into();
        for t in ts {
            res |= self.pieces(*t);
        }
        res
    }
    pub fn spec(&self, t: PieceType, c: Color) -> Bitboard {
        self.pieces(t) & self.color(c)
    }
    pub fn spec_list(&self, ts: &[PieceType], c: Color) -> Bitboard {
        self.pieces_list(ts) & self.color(c)
    }

    pub const fn piece_on(&self, s: Square) -> Option<Piece> {
        self.board[s as usize]
    }
    pub const fn empty(&self, s: Square) -> bool {
        self.board[s as usize].is_none()
    }

    pub fn king(&self, color: Color) -> Square {
        assert_ne!(self.spec(PieceType::King, color), Bitboard::new(0));
        // SAFETY: King always has to exist.
        unsafe { self.spec(PieceType::King, color).lsb_unchecked() }
    }

    // Castling
    pub fn has_castle(&self, cf: CastleFlag) -> bool {
        let cf_u8: u8 = cf.into();
        self.state().castle_rights & cf_u8 == cf_u8
    }
    pub fn can_castle(&self, cf: CastleFlag) -> bool {
        strict_not!(self.has_castle(cf), return false);

        // XXX Should this check more than just plegal?
        let inb = Bitboard::interval(cf.from_square(), cf.rook_from_square());
        if bool::from(inb & self.all()) {
            return false;
        }

        true
    }

    // State access, and mutations
    pub fn state(&self) -> &State {
        self.state.as_ref().unwrap()
    }
    fn state_mut(&mut self) -> &mut State {
        self.state.as_mut().unwrap()
    }

    // Non-setting access
    pub fn ep(&self) -> Option<Square> {
        self.state().en_passant
    }
    pub fn checkers(&self) -> Bitboard {
        self.state().checkers
    }
    pub fn pinners(&self, color: Color) -> Bitboard {
        self.state().pinners[color as usize]
    }
    pub fn blockers(&self, color: Color) -> Bitboard {
        self.state().blockers[color as usize]
    }
    pub fn rule50(&self) -> i32 {
        self.state().halfmoves
    }

    // Move related
    pub fn is_legal(&self, mov: Move) -> bool {
        strict_not!(self.is_pseudo_legal(mov), return false);

        todo!();
    }
    pub fn is_pseudo_legal(&self, mov: Move) -> bool {
        todo!()
    }
    pub fn make_move(&mut self, mov: Move) {
        strict_cond!(self.is_legal(mov));

        let mut new_state = self.state.clone().unwrap();
        let old = self.state.replace(new_state);
        self.state_mut().previous = old;

        self.state_mut().halfmoves += 1;

        let us = self.to_move();
        let them = !us;

        let from = mov.from();
        let to = mov.to();
        let flag = mov.kind();

        strict_ne!(from, to);

        let mover = self
            .piece_on(from)
            .expect("No piece found on the `from` square");

        strict_eq!(mover.color(), us);

        // This is the square we want to REMOVE a piece from after this.
        let mut capture_square = to;

        if mover.kind() == PieceType::Pawn {
            self.state_mut().halfmoves = 0;

            if from.distance(to) == 2 {
                strict_eq!(from.file(), to.file());
                self.state_mut().en_passant =
                    Some(Square::new(from.file(), us.relative_rank(Rank::Three)));
            } else if flag == MoveKind::EnPassant {
                strict_eq!(
                    self.state()
                        .previous
                        .as_ref()
                        .map(|st| st.en_passant)
                        .flatten(),
                    Some(to)
                );

                capture_square = Square::new(from.file(), us.relative_rank(Rank::Five));
            } else if let MoveKind::Promotion(promo_type) = flag {
                strict_ne!(promo_type, PieceType::Pawn);
                strict_ne!(promo_type, PieceType::King);
                self.remove_piece(from);
                self.add_piece(Piece::new(promo_type, us), from);
            }
        }

        if let Some(piece) = self.remove_piece(capture_square) {
            self.state_mut().halfmoves = 0;
            self.state_mut().captured = Some(piece);
        }

        self.move_piece(from, to);

        if flag == MoveKind::Castle {
            // We have to find our castle-flag first.
            let castle_flag = if CastleFlag::short_for(us).to_square() == to {
                CastleFlag::short_for(us)
            } else {
                CastleFlag::long_for(us)
            };

            strict_eq!(castle_flag.to_square(), to);
            strict_eq!(castle_flag.from_square(), from);

            self.move_piece(castle_flag.rook_from_square(), castle_flag.rook_to_square());
        }

        // TODO what is most efficient way? no checks?
        if mover.kind() == PieceType::King {
            for cf in CastleFlag::variants_for(us) {
                if self.has_castle(cf) {
                    self.remove_castle_right(cf);
                }
            }
        } else if mover.kind() == PieceType::Rook {
            for cf in CastleFlag::variants_for(us) {
                if cf.rook_from_square() == from && self.has_castle(cf) {
                    self.remove_castle_right(cf);
                }
            }
        }

        self.update_state();
    }
    pub fn unmake_move(&mut self, mov: Move) {
        todo!()
    }

    // Rest private helpers
    fn add_piece(&mut self, piece: Piece, square: Square) {
        if self.board[square as usize].is_some() {
            panic!("Position::add_piece: Square already occupied");
        }

        self.board[square as usize] = Some(piece);
        let bb = Bitboard::from(square);

        self.colors[piece.color() as usize] |= bb;
        self.pieces[piece.kind() as usize] |= bb;
    }
    #[must_use]
    fn remove_piece(&mut self, square: Square) -> Option<Piece> {
        let pc = self.board[square as usize].take()?;

        let bb = Bitboard::from(square);

        self.colors[pc.color() as usize] ^= bb;
        self.pieces[pc.kind() as usize] ^= bb;

        strict_cond!(self.piece_on(square).is_none());

        Some(pc)
    }
    fn move_piece(&mut self, from: Square, to: Square) {
        strict_ne!(from, to);
        strict_not!(self.piece_on(to).is_some());
        strict_cond!(self.piece_on(from).is_some());

        let x = Bitboard::from([from, to]);
        let pc = self.board[from as usize].take().unwrap();
        self.board[to as usize] = Some(pc);
        self.colors[pc.color() as usize] ^= x;
        self.pieces[pc.kind() as usize] ^= x;
    }

    fn add_castle_right(&mut self, cf: CastleFlag) {
        // Safety:: this is only used in Position::new_from_fen - state ref can't be invalidated and is released immediately.
        unsafe {
            self.state_mut().castle_rights |= u8::from(cf);
        }
    }
    fn remove_castle_right(&mut self, cf: CastleFlag) {
        // Safety:: this is only used in Position::new_from_fen - state ref can't be invalidated and is released immediately.
        unsafe {
            self.state_mut().castle_rights &= !u8::from(cf);
        }
    }

    fn attacks_to(&self, square: Square, by: Color) -> Bitboard {
        self.attacks_to_with_occ(square, by, self.all())
    }
    fn attacks_to_with_occ(&self, square: Square, by: Color, occupancy: Bitboard) -> Bitboard {
        let pawns = precompute::pawn_attacks(square, !by) & self.pieces(PieceType::Pawn);

        let knights = precompute::knight_attacks(square) & self.pieces(PieceType::Knight);

        // Both sliders calculate Queen moves in tandem
        let bishops = precompute::bishop_attacks(square, occupancy)
            & self.pieces_list(&[PieceType::Bishop, PieceType::Queen]);
        let rooks = precompute::rook_attacks(square, occupancy)
            & self.pieces_list(&[PieceType::Rook, PieceType::Queen]);

        let kings = precompute::king_attacks(square) & self.pieces(PieceType::King);

        let moves = pawns | knights | bishops | rooks | kings;

        moves & self.color(by)
    }

    fn update_state(&mut self) {
        let mov_color = self.to_move();
        let king = self.king(mov_color);

        self.state_mut().checkers = self.attacks_to(king, !mov_color);

        self.update_checkers_blockers(Color::White);
        self.update_checkers_blockers(Color::Black);
    }
    fn update_checkers_blockers(&mut self, color: Color) {
        let king = self.king(color);
        // TODO Is it SUBSTANTIALLY better to just have slider attacks calculated separately to avoid overhead of pawn/king/knight generations?
        let potential_pinners = self.attacks_to_with_occ(king, !color, 0.into())
            & self.pieces_list(&[PieceType::Bishop, PieceType::Rook, PieceType::Queen]);

        for pp in potential_pinners {
            let line_to_king = precompute::line(king, pp) & self.all();
            if line_to_king.more_than_one() || !bool::from(line_to_king) {
                // Not a pinner!!
                continue;
            }

            self.state_mut().blockers[color as usize] |= line_to_king;
            self.state_mut().pinners[(!color) as usize] |= Bitboard::from(pp);
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new_from_fen(Self::STARTING_FEN)
    }
}

impl State {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            blockers: [Bitboard::new(0); 2],
            pinners: [Bitboard::new(0); 2],
            checkers: Bitboard::new(0),
            captured: None,
            castle_rights: 0,
            en_passant: None,
            halfmoves: 0,
            previous: None,
        })
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self {
            captured: None,
            en_passant: None,
            pinners: [Bitboard::new(0); 2],
            blockers: [Bitboard::new(0); 2],
            checkers: Bitboard::new(0),

            halfmoves: self.halfmoves,
            castle_rights: self.castle_rights,

            previous: None,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pos_str = String::new();

        for fake_rank_index in 0..8 {
            pos_str += "+---+---+---+---+---+---+---+---+\n";
            pos_str += "| ";
            let rank_index = 7 - fake_rank_index;
            for file_index in 0..8 {
                // SAFETY: In proper range as declared.
                let f = unsafe { File::try_from(file_index).unwrap_unchecked() };
                let r = unsafe { Rank::try_from(rank_index).unwrap_unchecked() };
                let s = Square::new(f, r);
                pos_str.push(match self.piece_on(s) {
                    Some(p) => char::from(p),
                    None => ' ',
                });
                if file_index != 7 {
                    pos_str += " | ";
                }
            }
            pos_str += " |\n";
        }
        pos_str += "+---+---+---+---+---+---+---+---+\nEP: ";

        write!(
            f,
            "{pos_str}{}",
            match self.ep() {
                Some(s) => s.to_string(),
                None => "n/a".to_owned(),
            }
        )
    }
}
