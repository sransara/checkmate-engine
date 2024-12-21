#[derive(Debug, Clone, Copy)]
enum Color {
    White,
    Black,
}

impl Color {
    fn from_char(c: char) -> Color {
        match c {
            // lowercase
            'w' => Color::White,
            'b' => Color::Black,
            // uppercase
            'W' => Color::White,
            'B' => Color::Black,
            _ => panic!("invalid color"),
        }
    }

    fn from_str(s: &str) -> Color {
        let c = s.chars().next().expect("expected character");
        Color::from_char(c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    fn from_char(c: char) -> PieceKind {
        match c {
            // lowercase
            'p' => PieceKind::Pawn,
            'n' => PieceKind::Knight,
            'b' => PieceKind::Bishop,
            'r' => PieceKind::Rook,
            'q' => PieceKind::Queen,
            'k' => PieceKind::King,
            // uppercase
            'P' => PieceKind::Pawn,
            'N' => PieceKind::Knight,
            'B' => PieceKind::Bishop,
            'R' => PieceKind::Rook,
            'Q' => PieceKind::Queen,
            'K' => PieceKind::King,
            _ => panic!("invalid piece kind"),
        }
    }

    fn from_str(s: &str) -> PieceKind {
        let c = s.chars().next().expect("expected character");
        PieceKind::from_char(c)
    }
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    color: Color,
    kind: PieceKind,
}

impl Piece {
    fn from_char(c: char) -> Piece {
        match c {
            // lowercase
            'p' => Piece {
                color: Color::Black,
                kind: PieceKind::Pawn,
            },
            'n' => Piece {
                color: Color::Black,
                kind: PieceKind::Knight,
            },
            'b' => Piece {
                color: Color::Black,
                kind: PieceKind::Bishop,
            },
            'r' => Piece {
                color: Color::Black,
                kind: PieceKind::Rook,
            },
            'q' => Piece {
                color: Color::Black,
                kind: PieceKind::Queen,
            },
            'k' => Piece {
                color: Color::Black,
                kind: PieceKind::King,
            },
            // uppercase
            'P' => Piece {
                color: Color::White,
                kind: PieceKind::Pawn,
            },
            'N' => Piece {
                color: Color::White,
                kind: PieceKind::Knight,
            },
            'B' => Piece {
                color: Color::White,
                kind: PieceKind::Bishop,
            },
            'R' => Piece {
                color: Color::White,
                kind: PieceKind::Rook,
            },
            'Q' => Piece {
                color: Color::White,
                kind: PieceKind::Queen,
            },
            'K' => Piece {
                color: Color::White,
                kind: PieceKind::King,
            },
            _ => panic!("invalid piece"),
        }
    }

    fn from_str(s: &str) -> Piece {
        let c = s.chars().next().expect("expected character");
        Piece::from_char(c)
    }
}

#[derive(Debug)]
struct Square {
    file: i32,
    rank: i32,
}

impl Square {
    fn from_str(s: &str) -> Square {
        let file = s.chars().nth(0).expect("expected character") as i32 - 'a' as i32;
        let rank = s.chars().nth(1).expect("expected character") as i32 - '1' as i32;
        Square { file, rank }
    }
}

#[derive(Debug)]
struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl CastlingRights {
    fn from_str(s: &str) -> CastlingRights {
        let mut white_king_side = false;
        let mut white_queen_side = false;
        let mut black_king_side = false;
        let mut black_queen_side = false;

        for c in s.chars() {
            match c {
                'K' => white_king_side = true,
                'Q' => white_queen_side = true,
                'k' => black_king_side = true,
                'q' => black_queen_side = true,
                _ => panic!("invalid castling rights"),
            }
        }

        CastlingRights {
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side,
        }
    }
}

#[derive(Debug)]
struct Board {
    raw: [[Option<Piece>; 8]; 8],
}

impl Board {
    fn new() -> Board {
        const EMPTY: Option<Piece> = None;
        const FILES: [Option<Piece>; 8] = [EMPTY; 8];
        Board { raw: [FILES; 8] }
    }

    fn from_fen_board(fen_board: &str) -> Board {
        let mut board = Board::new();
        let mut rank = 7;
        let mut file = 0;
        for c in fen_board.chars() {
            match c {
                '/' => {
                    rank -= 1;
                    file = 0;
                }
                '1'..='8' => {
                    let n = c as i32 - '0' as i32;
                    for _ in 0..n {
                        board.raw[rank as usize][file as usize] = None;
                        file += 1;
                    }
                }
                _ => {
                    board.raw[rank as usize][file as usize] = Some(Piece::from_char(c));
                    file += 1;
                }
            }
        }
        board
    }
}

#[derive(Debug)]
struct GameState {
    board: Board,
    turn: Color,
    castling_rights: CastlingRights,
    en_passant: Option<Square>,
    moves: Vec<Move>,
    halfmove: i32,
    fullmove: i32,
}

#[derive(Debug)]
struct Move {
    start: Square,
    end: Square,
    end_piece: Option<PieceKind>, // when start piece is not the end piece
}

impl Move {
    fn from_uci(s: &str) -> Move {
        let mut chars = s.chars();

        let start: String = chars.by_ref().take(2).collect();
        let start = Square::from_str(&start);

        let end: String = chars.by_ref().take(2).collect();
        let end = Square::from_str(&end);

        let end_piece: String = chars.collect();
        let end_piece = match end_piece.as_str() {
            "" => None,
            s => Some(PieceKind::from_str(s)),
        };

        Move {
            start,
            end,
            end_piece,
        }
    }
}

impl GameState {
    fn from_fen(fen: &str) -> GameState {
        let mut parts = fen.split_whitespace();

        let board = parts.next().expect("expected board");
        let turn = parts.next().expect("expected turn");
        let castling_rights = parts.next().expect("expected castling_rights");
        let en_passant = parts.next().expect("expected en passant");
        let halfmove = parts.next().expect("expected halfmove");
        let fullmove = parts.next().expect("expected fullmove");

        let board = Board::from_fen_board(board);
        let turn = Color::from_str(turn);
        let castling_rights = match castling_rights {
            "-" => CastlingRights::from_str(""),
            s => CastlingRights::from_str(s)
        };
        let en_passant = match en_passant {
            "-" => None,
            s => Some(Square::from_str(s)),
        };
        let moves = Vec::new();
        let halfmove = halfmove.parse().expect("expected integer");
        let fullmove = fullmove.parse().expect("expected integer");

        GameState {
            board,
            turn,
            castling_rights,
            en_passant,
            moves,
            halfmove,
            fullmove,
        }
    }

    fn make_move(&mut self, move_: &Move) {
        let start_piece = &self.board.raw[move_.start.rank as usize][move_.start.file as usize];
        let start_piece = start_piece.expect("expected piece");
        let end_piece = match move_.end_piece {
            Some(piece) => piece,
            None => start_piece.kind,
        };
        self.board.raw[move_.end.rank as usize][move_.end.file as usize] = Some(Piece {
            color: start_piece.color,
            kind: end_piece,
        });
        self.board.raw[move_.start.rank as usize][move_.start.file as usize] = None;
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        if start_piece.kind == PieceKind::Pawn {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }
        self.fullmove += 1;
    }
}

fn main() {
    println!("ready");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    let mut game_state = GameState::from_fen(&input);

    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        let move_ = Move::from_uci(&input);
        game_state.make_move(&move_);
    }
}