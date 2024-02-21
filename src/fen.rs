use std::str::FromStr;

use log::debug;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::Color::{Black, White};
use crate::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::{fen, ChessBoard, Color, ParseError, Piece, BOARD_SIZE};

pub const INITIAL_FEN_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0";

#[derive(Parser)]
#[grammar = "fen.pest"]
struct FENParser;

pub trait FENStringParsing {
    fn parse_fen(&self) -> Result<ChessBoard, ParseError>;
}

trait ToPiece {
    fn to_piece(&self) -> Option<Piece>;
}

impl ToPiece for Pair<'_, Rule> {
    fn to_piece(&self) -> Option<Piece> {
        match self.as_rule() {
            Rule::white_bishop => Some(Piece::new(White, Bishop)),
            Rule::white_king => Some(Piece::new(White, King)),
            Rule::white_knight => Some(Piece::new(White, Knight)),
            Rule::white_pawn => Some(Piece::new(White, Pawn)),
            Rule::white_queen => Some(Piece::new(White, Queen)),
            Rule::white_rook => Some(Piece::new(White, Rook)),
            Rule::black_bishop => Some(Piece::new(Black, Bishop)),
            Rule::black_king => Some(Piece::new(Black, King)),
            Rule::black_knight => Some(Piece::new(Black, Knight)),
            Rule::black_pawn => Some(Piece::new(Black, Pawn)),
            Rule::black_queen => Some(Piece::new(Black, Queen)),
            Rule::black_rook => Some(Piece::new(Black, Rook)),
            _ => {
                debug!(
                    "{:?} Cannot be converted to any chess piece",
                    self.as_rule()
                );
                return None;
            }
        }
    }
}

impl FENStringParsing for str {
    fn parse_fen(&self) -> Result<ChessBoard, ParseError> {
        let parsed_fen = match FENParser::parse(fen::Rule::fen_board, &self) {
            Ok(mut pairs) => pairs.next().unwrap(),
            Err(e) => {
                eprintln!("Invalid FEN string {}", e);
                return Err(ParseError::InvalidFENString);
            }
        };
        let mut board = ChessBoard::new();
        let mut row = 0;
        for p0 in parsed_fen.into_inner() {
            match p0.as_rule() {
                Rule::rank => {
                    let mut col = 0;
                    for p1 in p0.into_inner() {
                        let p2 = p1.into_inner().peek().unwrap();
                        if p2.as_rule() == Rule::empty_squares {
                            let blanks = p2
                                .as_str()
                                .parse::<usize>()
                                .expect("Empty squares shouuld be a number between 1 and 8");
                            for _ in 0..blanks {
                                board.set_piece_0(row, col, None);
                                col += 1;
                            }
                        } else {
                            board.set_piece_0(row, col, p2.to_piece());
                            col += 1;
                        }
                    }
                }
                Rule::RANK_SEPARATOR => {
                    row += 1;
                }
                Rule::active_color => {
                    board.active_color = Color::from_str(p0.as_str())
                        .expect("Active color should be either 'b' or 'w'");
                }
                Rule::en_passant_square => {
                    board.passant_square = board.get_square_a(p0.as_str());
                }
                Rule::half_moves => {
                    board.half_moves = p0
                        .as_str()
                        .parse()
                        .expect("Half moves should be an integer");
                }
                Rule::full_moves => {
                    board.full_moves = p0
                        .as_str()
                        .parse()
                        .expect("Full moves should be an integer");
                }
                _ => {
                    debug!("Ignoring rule {:?}", p0.as_rule());
                }
            }
        }
        Ok(board)
    }
}

impl FromStr for ChessBoard {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        s.parse_fen()
    }
}

pub trait BoardAsFEN {
    fn as_fen(&self) -> String;
}

impl BoardAsFEN for ChessBoard {
    fn as_fen(&self) -> String {
        let mut fen_code = String::new();
        for rank in 0..BOARD_SIZE {
            let mut empty_squares = 0;
            for file in 0..BOARD_SIZE {
                let square = self.squares[rank][file];
                if square.is_empty() {
                    empty_squares += 1;
                    continue;
                }
                if empty_squares > 0 {
                    fen_code.push(char::from_digit(empty_squares, 10).unwrap());
                    empty_squares = 0;
                }
                fen_code.push(square.as_fen())
            }
            if empty_squares > 0 {
                fen_code.push(char::from_digit(empty_squares, 10).unwrap());
            }
            if rank < BOARD_SIZE - 1 {
                fen_code.push('/');
            }
        }
        fen_code.push_str(&*format!(
            " {} {} {} {} {}",
            self.active_color,
            self.get_castling_as_string(),
            match self.passant_square {
                None => '-',
                Some(_) => self.passant_square.unwrap().as_fen(),
            },
            self.half_moves,
            self.full_moves
        ));
        fen_code
    }
}
