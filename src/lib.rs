use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

use colored::Colorize;

pub use crate::fen::{BoardAsFEN, FENStringParsing, INITIAL_FEN_BOARD};
pub use crate::pgn::{PGNGame, PieceMove};
use crate::ChessMove::{CastleKingside, CastleQueenside};
use crate::Color::{Black, White};
use crate::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

mod fen;
mod pgn;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyString,
    StringTooShort,
    InvalidFENString,
    InvalidPosition,
    InvalidPositionRank,
    InvalidPositionFile,
    UselessMove,
    InvalidAlgebraicPosition,
}

pub const BOARD_SIZE: usize = 8;
// Chessboard is 8x8
pub const BOARD_SIZE_RANGE_0: Range<usize> = 0..BOARD_SIZE;
const BOARD_SIZE_RANGE_1: RangeInclusive<usize> = 1..=BOARD_SIZE;
const RANK_BASE_U8: u8 = '1' as u8;
const FILE_BASE_U8: u8 = 'a' as u8;
const RANK_UNICODE_USIZE_RANGE: RangeInclusive<usize> = 49..=56;
const FILE_USIZE_RANGE: RangeInclusive<usize> = 97..=104;
const FILE_CHAR_RANGE: RangeInclusive<char> = 'a'..='h';
pub const DEFAULT_KING_COL: usize = 4;
pub const DEFAULT_KINGSIDE_ROOK_COL: usize = 7;
pub const DEFAULT_QUEENSIDE_ROOK_COL: usize = 0;

/// The two different colors for the pieces
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn inverse(&self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorError;

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseColorError);
        }
        let c: char = s.to_lowercase().chars().next().unwrap();
        match c {
            'w' => Ok(Color::White),
            'b' => Ok(Color::Black),
            _ => Err(ParseColorError),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => {
                write!(f, "w")
            }
            Color::Black => {
                write!(f, "b")
            }
        }
    }
}

// Define an enum for the types of chess pieces
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// A Chess piece
///
/// The `Piece` structs represents an individual chess piece within the board. Besides it's basic
/// properties such as the `color` and `piece_type`, it also knows it's position on the board as the
/// `column` and `row` (As of now, they are zero based). It also has a `moves` counter, useful for
/// moves such as castling.
///
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub moves: u32,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Piece {
            color,
            piece_type,
            moves: 0,
        }
    }

    pub fn as_fen(&self) -> char {
        match self.color {
            Color::White => match self.piece_type {
                PieceType::Pawn => 'P',
                PieceType::Knight => 'N',
                PieceType::Bishop => 'B',
                PieceType::Rook => 'R',
                PieceType::Queen => 'Q',
                PieceType::King => 'K',
            },
            Color::Black => match self.piece_type {
                PieceType::Pawn => 'p',
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook => 'r',
                PieceType::Queen => 'q',
                PieceType::King => 'k',
            },
        }
    }

    pub fn new_from_algebraic(color: Color, p: &str) -> Self {
        Piece {
            color,
            moves: 0,
            piece_type: match p {
                "K" => King,
                "Q" => Queen,
                "R" => Rook,
                "B" => Bishop,
                "N" => Knight,
                _ => Pawn,
            },
        }
    }

    pub fn to_unicode_symbol(&self) -> char {
        match self.color {
            White => match self.piece_type {
                King => '♔',
                Queen => '♕',
                Rook => '♖',
                Bishop => '♗',
                Knight => '♘',
                Pawn => '♙',
            },
            Black => match self.piece_type {
                King => '♚',
                Queen => '♛',
                Rook => '♜',
                Bishop => '♝',
                Knight => '♞',
                Pawn => '♟',
            },
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece_name = match self.piece_type {
            Rook => "rook",
            Bishop => "bishop",
            Knight => "knight",
            Pawn => "pawn",
            Queen => "queen",
            King => "king",
        };
        write!(f, "{} {}", self.color.to_string(), piece_name)
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.piece_type == other.piece_type
    }
}

/// A Square in the board that can contain a [Piece]
///
/// A board is a collection of 64([BOARD_SIZE]) Squares. Each Square knows it's location on the board and comes with a
/// few convenience methods. See below.
///
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Square {
    pub piece: Option<Piece>,
    // Rank is a number between 1 and 8
    pub rank: usize,
    // File is a char between 'a' and 'h'
    pub file: char,
    // The zero-based row of this square
    pub row: usize,
    // The zero-based column of this square
    pub col: usize,
}

/// Represents a board square that must be highlighted by display routines
pub struct HighlightedSquare {
    /// row position of the square on the board (zero based)
    pub row: usize,
    /// column position of the square on the board (zero based)
    pub col: usize,
    /// The color of the related piece
    pub color: Color,
}

impl Square {
    /// A convenience helper to let you know if the square is empty.
    /// ```
    /// use ajedrez::Square;
    /// let s = Square {piece: None, rank: 1, file: 'a', row:0, col:0};
    /// assert!(s.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.piece.is_none()
    }

    /// A convenience helper to let you know if the square is not empty.
    /// Note: Use `is()` If you want to know the actual piece contained in this square.
    /// ```
    /// use ajedrez::PieceType::Rook;
    /// use ajedrez::{Piece, Square};
    /// use ajedrez::Color::White;
    /// let s = Square {piece: Some(Piece::new(White, Rook)), rank: 1, file: 'a', row: 0, col:0};
    /// assert!(s.is_not_empty());
    /// ```
    pub fn is_not_empty(&self) -> bool {
        self.piece.is_some()
    }

    /// Check whether the current square has a piece of this type
    ///
    /// ```
    /// use ajedrez::PieceType::{King, Rook};
    /// use ajedrez::{Piece, Square};
    /// use ajedrez::Color::White;
    /// let s = Square {piece: Some(Piece::new(White, Rook)), rank: 1, file: 'a', row: 0, col:0};
    /// assert!(s.is(Rook));
    /// assert!(!s.is(King))
    /// ```
    pub fn is(&self, piece_type: PieceType) -> bool {
        self.piece.is_some() && self.piece.unwrap().piece_type == piece_type
    }

    /// Check whether the piece on the current square has not moved
    ///
    /// ```
    /// use ajedrez::PieceType::Rook;
    /// use ajedrez::{Piece, Square};
    /// use ajedrez::Color::White;
    /// let mut s = Square {
    ///     piece: Some(Piece{color: White, piece_type: Rook, moves:0}),
    ///     rank: 1,
    ///     file: 'a',
    ///     row: 0,
    ///     col:0
    /// };
    /// assert!(s.has_not_moved());
    /// s.piece.as_mut().unwrap().moves = 1;  // Simulate the piece was moved
    /// assert!(!s.has_not_moved());
    /// ```
    pub fn has_not_moved(&self) -> bool {
        self.piece.is_some() && self.piece.unwrap().moves == 0
    }

    /// Returns a FEN representation (a single char) of the contained piece,
    /// or a space (maybe not a very good idea ... IDK yet).
    ///
    /// ```
    /// use ajedrez::PieceType::Rook;
    /// use ajedrez::{Piece, Square};
    /// use ajedrez::Color::White;
    /// let s = Square {piece: Some(Piece::new(White, Rook)), rank: 1, file: 'a', row: 0, col:0};
    /// assert_eq!(s.as_fen(), 'R');
    /// ```
    pub fn as_fen(&self) -> char {
        match self.piece {
            Some(_) => self.piece.unwrap().as_fen(),
            None => ' ',
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.piece {
            None => write!(f, "-"),
            Some(_) => write!(f, "{}{}", self.rank, self.file),
        }
    }
}

/// A structure that states the castling availability of a chess board.
pub struct CastlingStatus {
    /// White player can castle **kingside**
    pub white_kingside: bool,
    /// White player can castle **queenside**
    pub white_queenside: bool,
    /// Black player can castle **kingside**
    pub black_kingside: bool,
    /// White player can castle **queenside**
    pub black_queenside: bool,
    /// When true this means that there is a possibility of castling even if there are pieces in between the king and
    /// the rook
    pub check_empty_rows: bool,
}

pub struct ChessBoard {
    squares: [[Square; BOARD_SIZE]; BOARD_SIZE],
    /// Active Color: The next field indicates whose turn it is to move. "w" means it is White's
    /// turn, and "b" means it is Black's turn.
    pub active_color: Color,
    /// Halfmoves counter: The number of halfmoves (or ply) since the last capture or pawn advance. This
    /// field is used for the fifty-move rule, which allows a player to claim a draw if no capture
    /// or pawn move has occurred in the last fifty moves.
    pub half_moves: u32,
    /// Fullmove Counter: This is a counter of the full moves in the game. It starts at 1 and
    /// increments after Black's move.
    pub full_moves: u32,
    /// En Passant Target Square: If there's a square where an en passant capture is possible, that
    /// square is noted here. It's recorded using algebraic notation (e.g., "e3").
    /// If there's no en passant target square, this is represented by a dash "-".
    pub passant_square: Option<Square>,

    /// Highlight specific squares. Useful for printing. (Move later to a display layer?)
    pub highlighted: HashMap<(usize, usize), Color>,
}

/// Converts a chess rank to a zero-based index
pub fn rank_to_index(rank: usize) -> usize {
    match BOARD_SIZE_RANGE_1.contains(&rank) {
        true => BOARD_SIZE - rank,
        false => panic!("Rank must be a number between 1 and 8, you provided {rank}"),
    }
}

/// Converts a chess file to a zero-based index
fn c_file_to_index(file: &char) -> usize {
    match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("File must be a letter between a and h, you provided {file}"),
    }
}

trait File2Index {
    fn file_to_zero_base_index(&self) -> Result<usize, String>;
}

impl File2Index for char {
    fn file_to_zero_base_index(&self) -> Result<usize, String> {
        match self {
            'a' => Ok(0),
            'b' => Ok(1),
            'c' => Ok(2),
            'd' => Ok(3),
            'e' => Ok(4),
            'f' => Ok(5),
            'g' => Ok(6),
            'h' => Ok(7),
            _ => Err(format!(
                "File must be a letter between a and h, you provided {}",
                self
            )),
        }
    }
}

impl File2Index for str {
    fn file_to_zero_base_index(&self) -> Result<usize, String> {
        self.chars()
            .next()
            .expect("File should have at least one character")
            .file_to_zero_base_index()
    }
}

trait Rank2Index {
    fn rank_to_zero_base_index(&self) -> Result<usize, String>;
}

impl Rank2Index for char {
    fn rank_to_zero_base_index(&self) -> Result<usize, String> {
        match self {
            '1' => Ok(0),
            '2' => Ok(1),
            '3' => Ok(2),
            '4' => Ok(3),
            '5' => Ok(4),
            '6' => Ok(5),
            '7' => Ok(6),
            '8' => Ok(7),
            _ => Err(format!(
                "Rank must be a digit between 1 and 8, you provided {}",
                self
            )),
        }
    }
}

impl Rank2Index for str {
    fn rank_to_zero_base_index(&self) -> Result<usize, String> {
        self.chars()
            .next()
            .expect("Rank should have at least one character")
            .rank_to_zero_base_index()
    }
}

/// Converts a position in algebraic notation into a tuple of 2 usize, suitable for indexing
/// squares/pieces on the `ChessBoard.squares`
///
pub fn pos_from_str(s: &str) -> Result<(usize, usize), ParseError> {
    let sb = s.as_bytes();
    if s.len() != 2 {
        return Err(ParseError::StringTooShort);
    }

    if !FILE_USIZE_RANGE.contains(&(sb[0] as usize)) {
        return Err(ParseError::InvalidPositionFile);
    }
    if !RANK_UNICODE_USIZE_RANGE.contains(&(sb[1] as usize)) {
        return Err(ParseError::InvalidPositionRank);
    }
    Ok((
        BOARD_SIZE - 1 - (sb[1] - RANK_BASE_U8) as usize,
        (sb[0] - FILE_BASE_U8) as usize,
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub castling: bool,
}

impl Move {
    /// The most usual way to create a move.
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Move {
            from,
            to,
            castling: false,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ChessMove {
    Simple,
    CastleKingside,
    CastleQueenside,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChessMoveError {
    OutOfBounds,
    StartPieceMissing,
    CastlingForbidden,
    WrongPieceColor,
    TooManyPossibleMoves,
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sb = s.as_bytes();
        if s.len() != 4 {
            return Err(ParseError::StringTooShort);
        }

        if !FILE_USIZE_RANGE.contains(&(sb[0] as usize))
            || !FILE_USIZE_RANGE.contains(&(sb[2] as usize))
        {
            return Err(ParseError::InvalidPositionFile);
        }
        if !RANK_UNICODE_USIZE_RANGE.contains(&(sb[1] as usize))
            || !RANK_UNICODE_USIZE_RANGE.contains(&(sb[3] as usize))
        {
            return Err(ParseError::InvalidPositionRank);
        }

        let mov = Move {
            from: (
                BOARD_SIZE - 1 - (sb[1] - RANK_BASE_U8) as usize,
                (sb[0] - FILE_BASE_U8) as usize,
            ),
            to: (
                BOARD_SIZE - 1 - (sb[3] - RANK_BASE_U8) as usize,
                (sb[2] - FILE_BASE_U8) as usize,
            ),
            castling: false,
        };
        if mov.from == mov.to {
            return Err(ParseError::UselessMove);
        }
        Ok(mov)
    }
}

impl ChessBoard {
    pub fn new() -> Self {
        let mut squares = [[Square::default(); BOARD_SIZE]; BOARD_SIZE];
        // Each square obj knows it's location
        for (row, rank) in BOARD_SIZE_RANGE_1.enumerate() {
            for (col, file) in FILE_CHAR_RANGE.enumerate() {
                squares[row][col] = Square {
                    piece: None,
                    rank,
                    file,
                    row,
                    col,
                };
            }
        }
        ChessBoard {
            squares,
            active_color: Color::White,
            full_moves: 0,
            half_moves: 0,
            passant_square: None,
            highlighted: HashMap::new(),
        }
    }

    pub fn set_piece(
        &mut self,
        rank: usize,
        file: &char,
        color: Color,
        piece_type: PieceType,
    ) -> &mut ChessBoard {
        let index_rank = rank_to_index(rank);
        let index_file = c_file_to_index(file);
        self.squares[index_rank][index_file].piece = Some(Piece::new(color, piece_type));
        self
    }

    /// Sets the piece at the specified square using zero-based-index row and col
    /// This method call can be chained like this:
    ///
    /// ```ignore
    /// board.set_piece(...)
    ///    .set_piece(...)
    ///     .set_piece(...);
    /// ```
    pub fn set_piece_0(&mut self, row: usize, col: usize, piece: Option<Piece>) -> &mut ChessBoard {
        self.squares[row][col].piece = piece;
        self
    }

    /// Gets the piece at the specified square using zero-based-index row and col
    pub fn get_piece_0(&self, row: usize, col: usize) -> Option<Piece> {
        self.squares[row][col].piece
    }

    /// Gets the piece at the specified square using algebraic notation
    pub fn get_piece_a(&self, coordinate: &str) -> Option<Piece> {
        match coordinate {
            // File a
            "a1" => self.squares[7][0].piece,
            "a2" => self.squares[6][0].piece,
            "a3" => self.squares[5][0].piece,
            "a4" => self.squares[4][0].piece,
            "a5" => self.squares[3][0].piece,
            "a6" => self.squares[2][0].piece,
            "a7" => self.squares[1][0].piece,
            "a8" => self.squares[0][0].piece,
            // file b
            "b1" => self.squares[7][1].piece,
            "b2" => self.squares[6][1].piece,
            "b3" => self.squares[5][1].piece,
            "b4" => self.squares[4][1].piece,
            "b5" => self.squares[3][1].piece,
            "b6" => self.squares[2][1].piece,
            "b7" => self.squares[1][1].piece,
            "b8" => self.squares[0][1].piece,
            // File c
            "c1" => self.squares[7][2].piece,
            "c2" => self.squares[6][2].piece,
            "c3" => self.squares[5][2].piece,
            "c4" => self.squares[4][2].piece,
            "c5" => self.squares[3][2].piece,
            "c6" => self.squares[2][2].piece,
            "c7" => self.squares[1][2].piece,
            "c8" => self.squares[0][2].piece,
            // File d
            "d1" => self.squares[7][3].piece,
            "d2" => self.squares[6][3].piece,
            "d3" => self.squares[5][3].piece,
            "d4" => self.squares[4][3].piece,
            "d5" => self.squares[3][3].piece,
            "d6" => self.squares[2][3].piece,
            "d7" => self.squares[1][3].piece,
            "d8" => self.squares[0][3].piece,
            // File e
            "e1" => self.squares[7][4].piece,
            "e2" => self.squares[6][4].piece,
            "e3" => self.squares[5][4].piece,
            "e4" => self.squares[4][4].piece,
            "e5" => self.squares[3][4].piece,
            "e6" => self.squares[2][4].piece,
            "e7" => self.squares[1][4].piece,
            "e8" => self.squares[0][4].piece,
            // File F
            "f1" => self.squares[7][5].piece,
            "f2" => self.squares[6][5].piece,
            "f3" => self.squares[5][5].piece,
            "f4" => self.squares[4][5].piece,
            "f5" => self.squares[3][5].piece,
            "f6" => self.squares[2][5].piece,
            "f7" => self.squares[1][5].piece,
            "f8" => self.squares[0][5].piece,
            // File h
            "g1" => self.squares[7][6].piece,
            "g2" => self.squares[6][6].piece,
            "g3" => self.squares[5][6].piece,
            "g4" => self.squares[4][6].piece,
            "g5" => self.squares[3][6].piece,
            "g6" => self.squares[2][6].piece,
            "g7" => self.squares[1][6].piece,
            "g8" => self.squares[0][6].piece,
            // File h
            "h1" => self.squares[7][7].piece,
            "h2" => self.squares[6][7].piece,
            "h3" => self.squares[5][7].piece,
            "h4" => self.squares[4][7].piece,
            "h5" => self.squares[3][7].piece,
            "h6" => self.squares[2][7].piece,
            "h7" => self.squares[1][7].piece,
            "h8" => self.squares[0][7].piece,
            _ => None,
        }
    }

    pub fn get_square_a(&self, coordinate: &str) -> Option<Square> {
        match coordinate {
            // File a
            "a1" => Some(self.squares[7][0]),
            "a2" => Some(self.squares[6][0]),
            "a3" => Some(self.squares[5][0]),
            "a4" => Some(self.squares[4][0]),
            "a5" => Some(self.squares[3][0]),
            "a6" => Some(self.squares[2][0]),
            "a7" => Some(self.squares[1][0]),
            "a8" => Some(self.squares[0][0]),
            // file b
            "b1" => Some(self.squares[7][1]),
            "b2" => Some(self.squares[6][1]),
            "b3" => Some(self.squares[5][1]),
            "b4" => Some(self.squares[4][1]),
            "b5" => Some(self.squares[3][1]),
            "b6" => Some(self.squares[2][1]),
            "b7" => Some(self.squares[1][1]),
            "b8" => Some(self.squares[0][1]),
            // File c
            "c1" => Some(self.squares[7][2]),
            "c2" => Some(self.squares[6][2]),
            "c3" => Some(self.squares[5][2]),
            "c4" => Some(self.squares[4][2]),
            "c5" => Some(self.squares[3][2]),
            "c6" => Some(self.squares[2][2]),
            "c7" => Some(self.squares[1][2]),
            "c8" => Some(self.squares[0][2]),
            // File d
            "d1" => Some(self.squares[7][3]),
            "d2" => Some(self.squares[6][3]),
            "d3" => Some(self.squares[5][3]),
            "d4" => Some(self.squares[4][3]),
            "d5" => Some(self.squares[3][3]),
            "d6" => Some(self.squares[2][3]),
            "d7" => Some(self.squares[1][3]),
            "d8" => Some(self.squares[0][3]),
            // File e
            "e1" => Some(self.squares[7][4]),
            "e2" => Some(self.squares[6][4]),
            "e3" => Some(self.squares[5][4]),
            "e4" => Some(self.squares[4][4]),
            "e5" => Some(self.squares[3][4]),
            "e6" => Some(self.squares[2][4]),
            "e7" => Some(self.squares[1][4]),
            "e8" => Some(self.squares[0][4]),
            // File f
            "f1" => Some(self.squares[7][5]),
            "f2" => Some(self.squares[6][5]),
            "f3" => Some(self.squares[5][5]),
            "f4" => Some(self.squares[4][5]),
            "f5" => Some(self.squares[3][5]),
            "f6" => Some(self.squares[2][5]),
            "f7" => Some(self.squares[1][5]),
            "f8" => Some(self.squares[0][5]),
            // File g
            "g1" => Some(self.squares[7][6]),
            "g2" => Some(self.squares[6][6]),
            "g3" => Some(self.squares[5][6]),
            "g4" => Some(self.squares[4][6]),
            "g5" => Some(self.squares[3][6]),
            "g6" => Some(self.squares[2][6]),
            "g7" => Some(self.squares[1][6]),
            "g8" => Some(self.squares[0][6]),
            // File h
            "h1" => Some(self.squares[7][7]),
            "h2" => Some(self.squares[6][7]),
            "h3" => Some(self.squares[5][7]),
            "h4" => Some(self.squares[4][7]),
            "h5" => Some(self.squares[3][7]),
            "h6" => Some(self.squares[2][7]),
            "h7" => Some(self.squares[1][7]),
            "h8" => Some(self.squares[0][7]),
            _ => None,
        }
    }

    // This indicates the castling rights for both White and Black. It uses the following characters:
    // "K" if White can castle kingside.
    // "Q" if White can castle queenside.
    // "k" if Black can castle kingside.
    // "q" if Black can castle queenside.
    // A dash "-" indicates that neither side can castle.
    // Examples:
    // "KQkq"  indicates that both sides can castle on both sides.
    // "-" Neither side can castle
    pub fn get_castling(&self, check_empty_rows: bool) -> CastlingStatus {
        CastlingStatus {
            white_kingside: self.can_castle(White, ChessMove::CastleKingside, check_empty_rows),
            white_queenside: self.can_castle(White, ChessMove::CastleQueenside, check_empty_rows),
            black_kingside: self.can_castle(Black, ChessMove::CastleKingside, check_empty_rows),
            black_queenside: self.can_castle(Black, ChessMove::CastleQueenside, check_empty_rows),
            check_empty_rows: check_empty_rows,
        }
    }

    pub fn get_castling_as_string(&self) -> String {
        let castling = self.get_castling(false);
        let mut s = String::new();
        if castling.white_kingside {
            s.push('K');
        }
        if castling.white_queenside {
            s.push('Q');
        }
        if castling.black_kingside {
            s.push('k');
        }
        if castling.black_queenside {
            s.push('q');
        }
        if s.is_empty() {
            s.push('-');
        }
        s
    }

    /// Returns an ascii-art like string representation of the current state of the board.
    pub fn as_str(&mut self) -> String {
        let mut b = String::from("");
        b.push_str("    a   b   c   d   e   f   g   h\n");
        b.push_str("  ┌───┬───┬───┬───┬───┬───┬───┬───┐\n");
        for row in BOARD_SIZE_RANGE_0 {
            b.push_str(&*format!("{} │", row));
            for col in BOARD_SIZE_RANGE_0 {
                let token = match self.squares[row][col].piece {
                    Some(piece) => {
                        if piece.color == White {
                            piece.to_unicode_symbol().to_string().yellow()
                        } else {
                            piece.to_unicode_symbol().to_string().blue()
                        }
                    },
                    None => ' '.to_string().into(),
                };
                if self.highlighted.contains_key(&(row, col)) {
                    if self.highlighted.get(&(row, col)).unwrap() == &White {
                        b.push_str(&*format!(" {} │", token.black().on_yellow()));
                    } else {
                        b.push_str(&*format!(" {} │", token.black().on_blue()));
                    }
                } else {
                    b.push_str(&*format!(" {} │", token));
                };
            }
            b.push_str(&*format!(" {}\n", BOARD_SIZE - row));
            if row < 7 {
                b.push_str("  ├───┼───┼───┼───┼───┼───┼───┼───┤\n")
            }
        }
        b.push_str("  └───┴───┴───┴───┴───┴───┴───┴───┘\n");
        b.push_str("    0   1   2   3   4   5   6   7\n");
        b
    }

    // Moves system
    pub fn generate_intrinsic_pawn_moves(&self, position: (usize, usize)) -> Vec<Move> {
        let mut moves = Vec::new();
        let (x, y) = position;

        // Get the pawn at the current position
        let pawn = match self.squares[x][y].piece {
            Some(p) => p,
            None => return moves, // No pawn, so no moves.
        };

        // Ensure that the piece is a pawn
        if pawn.piece_type != PieceType::Pawn {
            return moves; // Not a pawn, so no moves.
        }

        // Determine the direction depending on the pawn's color
        let mut direction: isize = if pawn.color == Color::White { -1 } else { 1 };

        // Can only move forward within the RANK range
        let mut fwd = (x as isize + direction) as usize;
        if BOARD_SIZE_RANGE_0.contains(&fwd) {
            if self.squares[fwd][y].is_empty() {
                moves.push(Move {
                    from: position,
                    to: (fwd, y),
                    castling: false,
                });
            }

            // Capture diagonally, to the left, except for first file/column
            if (y as isize - 1) > 0 {
                let left = y - 1;
                if BOARD_SIZE_RANGE_0.contains(&left) && !self.squares[fwd][left].is_empty() {
                    moves.push(Move {
                        from: position,
                        to: (fwd, left),
                        castling: false,
                    });
                }
            }

            // Capture diagonally, to the right, except for last file/column
            let right = y + 1;
            if BOARD_SIZE_RANGE_0.contains(&right) && !self.squares[fwd][right].is_empty() {
                moves.push(Move {
                    from: position,
                    to: (fwd, right),
                    castling: false,
                });
            }
        }

        // Initial two-square move
        // Make sure to check the pawn is in the initial position and
        // there's no piece two squares ahead.
        let initial_position = match pawn.color {
            Color::White => x == 6,
            Color::Black => x == 1,
        };
        if initial_position {
            direction *= 2;
            fwd = (x as isize + direction) as usize;
            if self.squares[x][y].piece.unwrap().moves == 0 && self.squares[fwd][y].is_empty() {
                moves.push(Move {
                    from: position,
                    to: (fwd, y),
                    castling: false,
                });
            }
        }

        // TODO: en passant
        moves
    }

    pub fn generate_intrinsic_knight_moves(&self, position: (usize, usize)) -> Vec<Move> {
        let (x, y) = position;
        let mut moves = Vec::new();

        // Get the knight at the current position
        let knight = match self.squares[x][y].piece {
            Some(p) => p,
            None => return moves, // No knight, so no moves.
        };

        // Ensure that the piece is a knight
        if knight.piece_type != PieceType::Knight {
            return moves; // Not a knight, so no moves.
        }

        let knight_moves = [
            (1, 2),
            (2, 1), // Moves to the right and up
            (-1, 2),
            (-2, 1), // Moves to the left and up
            (1, -2),
            (2, -1), // Moves to the right and down
            (-1, -2),
            (-2, -1), // Moves to the left and down
        ];

        for &(dx, dy) in knight_moves.iter() {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            // Verify the move is within the bounds of the board
            if !(new_x >= 0
                && new_x < BOARD_SIZE as isize
                && new_y >= 0
                && new_y < BOARD_SIZE as isize)
            {
                continue;
            }

            // Verify that the square is either empty or occupied by an opponent's piece
            let dest_square: Square = self.squares[new_x as usize][new_y as usize];
            if dest_square.is_empty() || dest_square.piece.unwrap().color != knight.color {
                moves.push(Move {
                    from: position,
                    to: (new_x as usize, new_y as usize),
                    castling: false,
                });
            }
        }

        moves
    }

    pub fn generate_intrinsic_bishop_moves(&self, position: (usize, usize)) -> Vec<Move> {
        let mut moves = Vec::new();
        // Get the bishop at the current position
        let bishop = match self.squares[position.0][position.1].piece {
            Some(p) => p,
            None => return moves, // No bishop, so no moves.
        };

        // Ensure that the piece is a bishop
        if bishop.piece_type != PieceType::Bishop {
            return moves; // Not a bishop, so no moves.
        }

        // Diagonal offsets
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

        // Iterate in all diagonal directions
        for &(dx, dy) in &directions {
            let (mut x, mut y) = position;

            loop {
                x = (x as isize).wrapping_add(dx) as usize;
                y = (y as isize).wrapping_add(dy) as usize;

                // Break the loop if the move is off the board
                if x >= BOARD_SIZE || y >= BOARD_SIZE {
                    break;
                }

                let current_position = (x, y);

                match self.squares[x][y].piece {
                    Some(piece) => {
                        // If there's a piece of the opposite color, it can be captured
                        if piece.color != bishop.color {
                            moves.push(Move {
                                from: position,
                                to: current_position,
                                castling: false,
                            });
                        }
                        break; // Stop moving in this direction whether a piece was captured or it's blocked
                    }
                    None => {
                        // If the square is empty, it's a valid move
                        moves.push(Move {
                            from: position,
                            to: current_position,
                            castling: false,
                        });
                    }
                }
            }
        }

        moves
    }

    pub fn generate_intrinsic_rook_moves(&self, position: (usize, usize)) -> Vec<Move> {
        let (mut x, mut y) = position;
        let mut moves = Vec::new();

        // Get the rook at the current position
        let rook = match self.squares[x][y].piece {
            Some(p) => p,
            None => return moves, // No rook, so no moves.
        };

        // Ensure that the piece is a rook
        if rook.piece_type != PieceType::Rook {
            return moves; // Not a rook, so no moves.
        }

        // Define the four possible directions in which a rook can move: up, down, left, right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for &(dx, dy) in &directions {
            (x, y) = position;
            loop {
                x = (x as isize).wrapping_add(dx) as usize;
                y = (y as isize).wrapping_add(dy) as usize;

                // Stop the loop if the new position is off the board
                if x >= BOARD_SIZE || y >= BOARD_SIZE {
                    break;
                }

                match self.squares[x][y].piece {
                    Some(piece) => {
                        // If there's a piece of the opposite color, it can be captured
                        if piece.color != rook.color {
                            moves.push(Move {
                                from: position,
                                to: (x, y),
                                castling: false,
                            });
                        }
                        // Whether it's a capture or not, the rook can't move past this piece
                        break;
                    }
                    None => {
                        // Add the move to the list if the square is empty
                        moves.push(Move {
                            from: position,
                            to: (x, y),
                            castling: false,
                        });
                    }
                }
            }
        }

        moves
    }

    /// This function generates the King's moves with 3 constraints: Chess board boundaries,
    /// piece color/capture, and empty squares.
    pub fn generate_intrinsic_king_moves(&self, position: (usize, usize)) -> Vec<Move> {
        let mut moves = Vec::new();
        let king = match self.squares[position.0][position.1].piece {
            Some(p) => p,
            None => return moves, // No king, so no moves.
        };

        // Ensure that the piece is a king
        if king.piece_type != PieceType::King {
            return moves; // Not a king, so no moves.
        }

        // The king cannot
        // Offsets for the eight possible moves a king can make
        let move_offsets = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for &(dx, dy) in &move_offsets {
            let new_x = position.0 as isize + dx;
            let new_y = position.1 as isize + dy;

            // Ensure the new coordinates are within the board boundaries
            if new_x >= 0
                && new_x < BOARD_SIZE as isize
                && new_y >= 0
                && new_y < BOARD_SIZE as isize
            {
                match self.squares[new_x as usize][new_y as usize].piece {
                    Some(piece) => {
                        // If the square is occupied by an opponent's piece, it's a capture move
                        if piece.color != king.color {
                            moves.push(Move {
                                from: position,
                                to: (new_x as usize, new_y as usize),
                                castling: false,
                            });
                        }
                        // Otherwise, the king cannot move into a square occupied by an allied piece
                    }
                    None => {
                        // If the square is unoccupied, it's a valid move
                        moves.push(Move {
                            from: position,
                            to: (new_x as usize, new_y as usize),
                            castling: false,
                        });
                    }
                }
            }
        }

        // Castling could be added here if you are checking board state, i.e., whether rooks and king
        // have moved, and if there is a clear path for castling.

        moves
    }

    pub fn generate_intrinsic_queen_moves(&self, position: (usize, usize)) -> Vec<Move> {
        let mut moves = Vec::new();

        let queen = match self.squares[position.0][position.1].piece {
            Some(p) => p,
            None => return moves, // No queen, so no moves.
        };

        // Ensure that the piece is a queen
        if queen.piece_type != PieceType::Queen {
            return moves; // Not a queen, so no moves.
        }

        // Directions combining both rook and bishop moves (horizontal, vertical, diagonal)
        let directions = [
            // Horizontal and vertical like a rook
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            // Diagonals like a bishop
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        for &(dx, dy) in directions.iter() {
            let (mut x, mut y) = position;

            loop {
                x = (x as isize).wrapping_add(dx) as usize;
                y = (y as isize).wrapping_add(dy) as usize;

                // Break loop if out of bounds
                if x >= BOARD_SIZE || y >= BOARD_SIZE {
                    break;
                }

                match self.squares[x][y].piece {
                    Some(piece) => {
                        // If a piece is found on the path
                        if piece.color != queen.color {
                            // If the piece is of opposite color, it can be captured
                            moves.push(Move {
                                from: position,
                                to: (x, y),
                                castling: false,
                            });
                        }
                        // Since a piece is on this square, the queen cannot move past; break the loop
                        break;
                    }
                    None => {
                        // No piece on the square, the queen can move here
                        moves.push(Move {
                            from: position,
                            to: (x, y),
                            castling: false,
                        });
                    }
                }
            }
        }

        moves
    }

    /// Generates the set of possible moves for a given position with the most basic constraints.
    pub fn generate_intrinsic_moves(&self, position: (usize, usize)) -> Vec<Move> {
        match self.squares[position.0][position.1]
            .piece
            .unwrap()
            .piece_type
        {
            Pawn => self.generate_intrinsic_pawn_moves(position),
            Knight => self.generate_intrinsic_knight_moves(position),
            Bishop => self.generate_intrinsic_bishop_moves(position),
            Rook => self.generate_intrinsic_rook_moves(position),
            King => self.generate_intrinsic_king_moves(position),
            Queen => self.generate_intrinsic_queen_moves(position),
        }
    }

    pub fn infer_move(&self, to_position: (usize, usize), piece_type: PieceType) -> Option<Move> {
        // Loop over all squares of the board to find opponent pieces
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[i][j].piece {
                    if piece.color == self.active_color && piece.piece_type == piece_type {
                        // Generate moves for this piece
                        for m in self.generate_intrinsic_moves((i, j)) {
                            if m.to == to_position {
                                return Some(m);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Returns a set of all targeted squares by all the pieces of the provided color
    pub fn targeted_squares(&self, color: Color) -> BTreeSet<(usize, usize)> {
        let mut squares = BTreeSet::new();
        // Loop over all squares of the board to find opponent pieces
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[i][j].piece {
                    if piece.color == color {
                        // Generate moves for this piece
                        for m in self.generate_intrinsic_moves((i, j)) {
                            // A move that directly targets king's position
                            // or a move that targets the boundaries of the king
                            squares.insert(m.to);
                        }
                    }
                }
            }
        }
        squares
    }

    pub fn find_pieces(&self, piece_type: PieceType, color: Color) -> Vec<&Square> {
        let mut squares: Vec<&Square> = Vec::new();
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[i][j].piece {
                    if piece.color == color && piece.piece_type == piece_type {
                        squares.push(&self.squares[i][j]);
                    }
                }
            }
        }
        squares
    }

    pub fn find_king(&self, king_color: Color) -> Option<(i32, i32)> {
        // Find the king's position
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if let Some(piece) = self.squares[i][j].piece {
                    if piece.color == king_color && piece.piece_type == PieceType::King {
                        Some((i, j));
                    }
                }
            }
        }
        None
    }

    /// Generates a set of King moves, i.e. intrinsic moves minus the squares where the king can be
    /// captured. If the resulting set is empty, we can conclude the king is in full check mate and
    /// the game is over.
    pub fn generate_constrained_king_moves(&mut self, position: (usize, usize)) -> Vec<Move> {
        let mut moves = Vec::new();
        if let Some(king) = self.squares[position.0][position.1].piece {
            // Ensure that the piece is a king
            if king.piece_type == PieceType::King {
                let intrinsic = self
                    .generate_intrinsic_king_moves(position)
                    .iter()
                    .map(|mov| mov.to)
                    .collect::<BTreeSet<(usize, usize)>>();
                let targeted = self.targeted_squares(if king.color == Color::White {
                    Color::Black
                } else {
                    Color::Black
                });
                let constrained = intrinsic
                    .difference(&targeted)
                    .cloned()
                    .collect::<Vec<(usize, usize)>>();
                for pos in constrained {
                    moves.push(Move {
                        from: position,
                        to: pos,
                        castling: false,
                    });
                }

                // if we still have moves left, remove the ones that would set the king into checkmate
                let moves_copy = moves.clone();
                moves.clear();
                if moves_copy.len() > 0 {
                    for m in moves_copy {
                        // Move the king to the new possible position
                        self.squares[m.from.0][m.from.1].piece = None;
                        self.squares[m.to.0][m.to.1].piece = Some(king);

                        // If the king is not in check, it is a good move
                        let targeted = self.targeted_squares(if king.color == Color::White {
                            Color::Black
                        } else {
                            Color::Black
                        });
                        if !targeted.contains(&position) {
                            moves.push(m);
                        }
                        // Return the king to it's original position
                        self.squares[m.to.0][m.to.1].piece = None;
                        self.squares[m.from.0][m.from.1].piece = Some(king);
                    }
                }

                if self.can_castle(king.color, ChessMove::CastleKingside, false) {
                    moves.push(Move {
                        from: position,
                        to: (position.0, 6),
                        castling: true,
                    })
                }
                if self.can_castle(king.color, CastleQueenside, false) {
                    moves.push(Move {
                        from: position,
                        to: (position.0, 2),
                        castling: true,
                    })
                }
            }
        }

        moves
    }

    /// A function that determines if the king is in check at the current position
    pub fn is_king_in_check(&self, position: (usize, usize)) -> bool {
        if let Some(king) = self.squares[position.0][position.1].piece {
            // Ensure that the piece is a king
            if king.piece_type == PieceType::King {
                let targeted = self.targeted_squares(king.color.inverse());
                return targeted.contains(&position);
            }
        }
        false // The king is not in check.
    }

    /// Moves the piece and increments the movements counter
    pub fn move_piece(&mut self, mov: Move) -> Result<String, ChessMoveError> {
        let (from_x, from_y) = mov.from;
        let (to_x, to_y) = mov.to;

        // Verify the move is within the bounds of the board
        if from_x >= BOARD_SIZE || from_y >= BOARD_SIZE || to_x >= BOARD_SIZE || to_y >= BOARD_SIZE
        {
            return Err(ChessMoveError::OutOfBounds);
        }

        // Ensure the start piece is not missing
        if self.squares[from_x][from_y].is_empty() {
            return Err(ChessMoveError::StartPieceMissing);
        }

        // Whose turn is it?
        let piece = self.squares[from_x][from_y].piece.unwrap();
        if piece.color != self.active_color {
            return Err(ChessMoveError::WrongPieceColor);
        }

        // Do we have capture?
        let action_str = if !self.squares[to_x][to_y].is_empty() {
            format!(
                "{:?} at ({}, {}) captures {:?} at ({}, {})",
                self.squares[from_x][from_y].piece.unwrap().piece_type,
                from_x,
                from_y,
                self.squares[to_x][to_y].piece.unwrap().piece_type,
                to_x,
                to_y
            )
        } else {
            format!(
                "{:?} at ({}, {}) moves to ({}, {})",
                self.squares[from_x][from_y].piece.unwrap().piece_type,
                from_x,
                from_y,
                to_x,
                to_y
            )
        };

        // Now move the piece
        self.squares[to_x][to_y].piece = self.squares[from_x][from_y].piece;
        self.squares[to_x][to_y].piece.unwrap().moves += 1;
        self.squares[from_x][from_y].piece = None;

        if self.squares[to_x][to_y].piece.unwrap().color == White {
            // Clear highlighted squares
            self.highlighted.clear();
        } else {
            // Increment full move only after the black moves
            self.full_moves += 1;
        }

        // Highlight the involved squares
        self.highlighted.insert((from_x, from_y), self.active_color);
        self.highlighted.insert((to_x, to_y), self.active_color);

        // Set the next active color
        self.active_color = self.active_color.inverse();
        Ok(action_str)
    }

    /// Analyzes the board to tell if the king at the given position can castle
    ///
    /// ## Castling rules:
    ///
    /// 1. Neither the king nor the chosen rook has previously moved during the game.
    /// 2. There are no pieces between the king and the chosen rook.
    /// 3. The king is not currently in check.
    /// 4. The squares that the king passes over are not attacked by an enemy piece, nor is the square where the king lands.
    /// 5. The king does not pass through a square that is attacked by an enemy piece.
    ///
    /// ## Parameters
    ///
    /// * `color`: The player color. Either Black or White
    /// * `castle_type`: Either `ChessMove::CastleKingside` or `CastleQueenside`. If you pass something else the method
    ///    will fail silently and return false.
    /// * `check_empty_squares`: If false it will ignore rule number #2
    ///
    /// ## Return
    ///
    /// True if the King can castle
    pub fn can_castle(
        &self,
        color: Color,
        castle_type: ChessMove,
        check_empty_squares: bool,
    ) -> bool {
        let row = if color == Color::Black { 0 } else { 7 };
        let rook_col: usize;
        let empty_squares: Vec<(usize, usize)>;
        match castle_type {
            ChessMove::CastleKingside => {
                rook_col = DEFAULT_KINGSIDE_ROOK_COL;
                empty_squares = vec![(row, DEFAULT_KING_COL + 1), (row, DEFAULT_KING_COL + 2)];
            }
            CastleQueenside => {
                rook_col = DEFAULT_QUEENSIDE_ROOK_COL;
                empty_squares = vec![
                    (row, DEFAULT_KING_COL - 1),
                    (row, DEFAULT_KING_COL - 2),
                    (row, DEFAULT_KING_COL - 3),
                ];
            }
            _ => return false,
        }

        if let (Some(king), Some(rook)) = (
            self.squares[row][DEFAULT_KING_COL].piece,
            self.squares[row][rook_col].piece,
        ) {
            let targeted = self.targeted_squares(king.color.inverse());
            // Ensure that the pieces are the right type ...
            return king.piece_type == PieceType::King && rook.piece_type == Rook
                // ... and color
                && rook.color == color && king.color == color
                // ... the king and the kingside rook haven't moved
                && king.moves == 0 && rook.moves == 0
                // ... the squares between them are empty,
                && if check_empty_squares {
                empty_squares.iter().all(|p| self.squares[p.0][p.1].is_empty())
            } else { true }
                && !targeted.contains(&(row, DEFAULT_KING_COL))
                //  ... doesn't move through check, and isn't castling into check.
                && ! empty_squares.iter().any(|p| targeted.contains(p));
        }
        false
    }

    /// Performs castling, constrained by the rules described on `can_castle()`
    pub fn castle(
        &mut self,
        color: Color,
        castle_type: ChessMove,
    ) -> Result<String, ChessMoveError> {
        let row = match color {
            White => 7,
            Black => 0,
        };

        let king_col = DEFAULT_KING_COL;
        let nw_king_col: usize;
        let rook_col: usize;
        let nw_rook_col: usize;

        match castle_type {
            ChessMove::CastleKingside => {
                nw_king_col = 6;
                rook_col = 7;
                nw_rook_col = 5;
            }
            CastleQueenside => {
                nw_king_col = 2;
                rook_col = 0;
                nw_rook_col = 3;
            }
            _ => {
                return Err(ChessMoveError::CastlingForbidden);
            }
        };

        if self.can_castle(color, castle_type, false) {
            let mut king = self.squares[row][king_col].piece.unwrap();
            king.moves += 1;
            let mut rook = self.squares[row][rook_col].piece.unwrap();
            rook.moves += 1;
            self.squares[row][king_col].piece = None;
            self.squares[row][nw_king_col].piece = Some(king);
            self.squares[row][rook_col].piece = None;
            self.squares[row][nw_rook_col].piece = Some(rook);

            if color == White {
                // Clear highlighted squares
                self.highlighted.clear();
            } else {
                self.full_moves += 1;
            }

            // Highlight the involved squares
            self.highlighted.insert((row, king_col), color);
            self.highlighted.insert((row, nw_king_col), color);
            self.highlighted.insert((row, rook_col), color);
            self.highlighted.insert((row, nw_rook_col), color);

            // Toggle the active color
            self.active_color = self.active_color.inverse();

            let msg = format!(
                "castles {}",
                if castle_type == CastleKingside {
                    "kingside"
                } else {
                    "queenside"
                }
            );
            return Ok(msg);
        }

        Err(ChessMoveError::CastlingForbidden)
    }
}
