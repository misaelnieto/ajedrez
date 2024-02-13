use std::collections::BTreeSet;
use std::fmt;
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

use colored::Colorize;
use regex::{Regex, RegexBuilder};

use crate::Color::{Black, White};
use crate::ParseError::EmptyString;
pub use crate::pgn::PGNGame;
use crate::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

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
            Black => White
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseColorError;

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() { return Err(ParseColorError); }
        let c: char = s.to_lowercase().chars().next().unwrap();
        match c {
            'w' => {
                Ok(Color::White)
            }
            'b' => {
                Ok(Color::Black)
            }
            _ => { Err(ParseColorError) }
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

// For convenience, we can implement a constructor for the Piece struct
impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Piece { color, piece_type, moves: 0 }
    }

    pub fn as_fen(&self) -> char {
        match self.color {
            Color::White => {
                match self.piece_type {
                    PieceType::Pawn => 'P',
                    PieceType::Knight => 'N',
                    PieceType::Bishop => 'B',
                    PieceType::Rook => 'R',
                    PieceType::Queen => 'Q',
                    PieceType::King => 'K',
                }
            }
            Color::Black => {
                match self.piece_type {
                    PieceType::Pawn => 'p',
                    PieceType::Knight => 'n',
                    PieceType::Bishop => 'b',
                    PieceType::Rook => 'r',
                    PieceType::Queen => 'q',
                    PieceType::King => 'k',
                }
            }
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
}


impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::White => {
                match self.piece_type {
                    PieceType::Pawn => write!(f, "♙"),
                    PieceType::Knight => write!(f, "♘"),
                    PieceType::Bishop => write!(f, "♗"),
                    PieceType::Rook => write!(f, "♖"),
                    PieceType::Queen => write!(f, "♕"),
                    PieceType::King => write!(f, "♔"),
                }
            }
            Color::Black => {
                match self.piece_type {
                    PieceType::Pawn => write!(f, "♚"),
                    PieceType::Knight => write!(f, "♞"),
                    PieceType::Bishop => write!(f, "♝"),
                    PieceType::Rook => write!(f, "♜"),
                    PieceType::Queen => write!(f, "♛"),
                    PieceType::King => write!(f, "♚"),
                }
            }
        }
    }
}


impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.piece_type == other.piece_type
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Square {
    pub piece: Option<Piece>,
    pub rank: usize,
    pub file: char,
}

impl Square {
    pub fn is_empty(&self) -> bool {
        self.piece.is_none()
    }

    /// Check whether the current square has a piece of this type
    pub fn is(&self, piece_type: PieceType) -> bool {
        self.piece.is_some() && self.piece.unwrap().piece_type == piece_type
    }

    /// Check whether the piece on the current square has not moved
    pub fn has_not_moved(&self) -> bool {
        self.piece.is_some() && self.piece.unwrap().moves == 0
    }


    pub fn as_fen(&self) -> char {
        match self.piece {
            Some(_) => self.piece.unwrap().as_fen(),
            None => ' ',
        }
    }

    pub fn to_zero_based_index(&self) -> (usize, usize) {
        (self.rank - 1, self.file.file_to_zero_base_index().unwrap())
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.piece {
            None => write!(f, "-"),
            Some(_) => write!(f, "{}{}", self.rank, self.file)
        }
    }
}


const FEN_REGEX: &str = concat!(
r"(?<board>[KQRBNP1-8\/]+)\s+",
r"(?<turn>[wb])\s+",
r"(?<castling>[KQ-]+)\s+",
r"(?<passant>[a-h1-8]+|\-)\s+",
r"(?<halfmove>\d+)\s+",
r"(?<fullmove>\d+)",
);


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

    /// Highlight specific squares. Useful for printing.
    highlighted: Vec<Square>,
}


/// Converts a chess rank to a zero-based index
pub fn rank_to_index(rank: usize) -> usize {
    match BOARD_SIZE_RANGE_1.contains(&rank) {
        true => BOARD_SIZE - rank,
        false => panic!("Rank must be a number between 1 and 8, you provided {rank}")
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
            _ => Err(format!("File must be a letter between a and h, you provided {}", self)),
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
            _ => Err(format!("Rank must be a digit between 1 and 8, you provided {}", self)),
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
        Move { from, to, castling: false }
    }
}

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
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sb = s.as_bytes();
        if s.len() != 4 {
            return Err(ParseError::StringTooShort);
        }

        if !FILE_USIZE_RANGE.contains(&(sb[0] as usize)) || !FILE_USIZE_RANGE.contains(&(sb[2] as usize)) {
            return Err(ParseError::InvalidPositionFile);
        }
        if !RANK_UNICODE_USIZE_RANGE.contains(&(sb[1] as usize)) || !RANK_UNICODE_USIZE_RANGE.contains(&(sb[3] as usize)) {
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
        for (i, rank) in BOARD_SIZE_RANGE_1.enumerate() {
            for (j, file) in FILE_CHAR_RANGE.enumerate() {
                squares[i][j] = Square { piece: None, rank, file };
            }
        }
        ChessBoard { squares, active_color: Color::White, full_moves: 0, half_moves: 0, passant_square: None, highlighted: vec![] }
    }

    pub fn set_piece(&mut self, rank: usize, file: &char, color: Color, piece_type: PieceType) -> &mut ChessBoard {
        let index_rank = rank_to_index(rank);
        let index_file = c_file_to_index(file);
        self.squares[index_rank][index_file].piece = Some(Piece::new(color, piece_type));
        self
    }

    pub fn get_piece(&self, rank: usize, file: usize) -> Option<Piece> {
        self.squares[rank][file].piece
    }

    /*
        This indicates the castling rights for both White and Black. It uses the following characters:
        "K" if White can castle kingside.
        "Q" if White can castle queenside.
        "k" if Black can castle kingside.
        "q" if Black can castle queenside.
        A dash "-" indicates that neither side can castle.
        Examples:
        "KQkq"  indicates that both sides can castle on both sides.
        "-" Neither side can castle
    */
    pub fn get_castling(&self) -> Vec<Piece> {
        vec![]
    }

    pub fn get_castling_as_string(&self) -> String {
        // Convert the vector of structs into a string representation.
        let castling: Vec<String> = self.get_castling().iter().map(|s| s.to_string()).collect();
        if castling.is_empty() {
            String::from("-")
        } else {
            castling.join("")
        }
    }

    /// Returns a ascii-art like string representation of the current state of the board.
    pub fn as_str(&mut self) -> String {
        let mut b = String::from("");
        b.push_str("    a   b   c   d   e   f   g   h\n");
        b.push_str("  ┌───┬───┬───┬───┬───┬───┬───┬───┐\n");
        for row in BOARD_SIZE_RANGE_0 {
            b.push_str(&*format!("{} │", row));
            for col in BOARD_SIZE_RANGE_0 {
                let token = match self.squares[row][col].piece {
                    Some(piece) => piece.to_string(),
                    None => " ".to_string(),
                    };
                let highlight_current = self.highlighted.iter()
                    .position(|s| s == &self.squares[row][col])
                    .map(|e| self.highlighted.remove(e))
                    .is_some();
                if highlight_current {
                    b.push_str(&*format!(" {} │", token.black().on_yellow()));
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

    pub fn as_fen(&self) -> String {
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
        fen_code.push_str(
            &*format!(" {} {} {} {} {}",
                      self.active_color,
                      self.get_castling_as_string(),
                      match self.passant_square {
                          None => '-',
                          Some(_) => self.passant_square.unwrap().as_fen()
                      },
                      self.half_moves,
                      self.full_moves)
        );
        fen_code
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
                moves.push(Move { from: position, to: (fwd, y), castling: false });
            }

            // Capture diagonally, to the left, except for first file/column
            if (y as isize - 1) > 0 {
                let left = y - 1;
                if BOARD_SIZE_RANGE_0.contains(&left) && !self.squares[fwd][left].is_empty() {
                    moves.push(Move { from: position, to: (fwd, left), castling: false });
                }
            }

            // Capture diagonally, to the right, except for last file/column
            let right = y + 1;
            if BOARD_SIZE_RANGE_0.contains(&right) && !self.squares[fwd][right].is_empty() {
                moves.push(Move { from: position, to: (fwd, right), castling: false });
            }
        }

        // Initial two-square move
        // Make sure to check the pawn is in the initial position and
        // there's no piece two squares ahead.
        let initial_position = match pawn.color {
            Color::White => { x == 6 }
            Color::Black => { x == 1 }
        };
        if initial_position {
            direction *= 2;
            fwd = (x as isize + direction) as usize;
            if self.squares[x][y].piece.unwrap().moves == 0 && self.squares[fwd][y].is_empty() {
                moves.push(Move { from: position, to: (fwd, y), castling: false });
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
            (1, 2), (2, 1),    // Moves to the right and up
            (-1, 2), (-2, 1),  // Moves to the left and up
            (1, -2), (2, -1),  // Moves to the right and down
            (-1, -2), (-2, -1) // Moves to the left and down
        ];

        for &(dx, dy) in knight_moves.iter() {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            // Verify the move is within the bounds of the board
            if !(new_x >= 0 && new_x < BOARD_SIZE as isize &&
                new_y >= 0 && new_y < BOARD_SIZE as isize) {
                continue;
            }

            // Verify that the square is either empty or occupied by an opponent's piece
            let dest_square: Square = self.squares[new_x as usize][new_y as usize];
            if dest_square.is_empty() || dest_square.piece.unwrap().color != knight.color {
                moves.push(Move { from: position, to: (new_x as usize, new_y as usize), castling: false });
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
                            moves.push(Move { from: position, to: (x, y), castling: false });
                        }
                        // Whether it's a capture or not, the rook can't move past this piece
                        break;
                    }
                    None => {
                        // Add the move to the list if the square is empty
                        moves.push(Move { from: position, to: (x, y), castling: false });
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
            (-1, -1), (0, -1), (1, -1),
            (-1, 0), (1, 0),
            (-1, 1), (0, 1), (1, 1),
        ];

        for &(dx, dy) in &move_offsets {
            let new_x = position.0 as isize + dx;
            let new_y = position.1 as isize + dy;

            // Ensure the new coordinates are within the board boundaries
            if new_x >= 0 && new_x < BOARD_SIZE as isize && new_y >= 0 && new_y < BOARD_SIZE as isize {
                match self.squares[new_x as usize][new_y as usize].piece {
                    Some(piece) => {
                        // If the square is occupied by an opponent's piece, it's a capture move
                        if piece.color != king.color {
                            moves.push(Move { from: position, to: (new_x as usize, new_y as usize), castling: false });
                        }
                        // Otherwise, the king cannot move into a square occupied by an allied piece
                    }
                    None => {
                        // If the square is unoccupied, it's a valid move
                        moves.push(Move { from: position, to: (new_x as usize, new_y as usize), castling: false });
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
            (-1, 0), (1, 0), (0, -1), (0, 1),
            // Diagonals like a bishop
            (-1, -1), (-1, 1), (1, -1), (1, 1),
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
                            moves.push(Move { from: position, to: (x, y), castling: false });
                        }
                        // Since a piece is on this square, the queen cannot move past; break the loop
                        break;
                    }
                    None => {
                        // No piece on the square, the queen can move here
                        moves.push(Move { from: position, to: (x, y), castling: false });
                    }
                }
            }
        }

        moves
    }

    /// Generates the set of possible moves for a given position with the most basic constraints.
    pub fn generate_intrinsic_moves(&self, position: (usize, usize)) -> Vec<Move> {
        match self.squares[position.0][position.1].piece.unwrap().piece_type {
            PieceType::Pawn => self.generate_intrinsic_pawn_moves(position),
            PieceType::Knight => self.generate_intrinsic_knight_moves(position),
            PieceType::Bishop => self.generate_intrinsic_bishop_moves(position),
            PieceType::Rook => self.generate_intrinsic_rook_moves(position),
            PieceType::King => self.generate_intrinsic_king_moves(position),
            PieceType::Queen => self.generate_intrinsic_queen_moves(position),
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
                let intrinsic = self.generate_intrinsic_king_moves(position)
                    .iter()
                    .map(|mov| mov.to)
                    .collect::<BTreeSet<(usize, usize)>>();
                let targeted = self.targeted_squares(if king.color == Color::White { Color::Black } else { Color::Black });
                let constrained = intrinsic.difference(&targeted)
                    .cloned()
                    .collect::<Vec<(usize, usize)>>();
                for pos in constrained {
                    moves.push(Move { from: position, to: pos, castling: false });
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
                        let targeted = self.targeted_squares(if king.color == Color::White { Color::Black } else { Color::Black });
                        if !targeted.contains(&position) {
                            moves.push(m);
                        }
                        // Return the king to it's original position
                        self.squares[m.to.0][m.to.1].piece = None;
                        self.squares[m.from.0][m.from.1].piece = Some(king);
                    }
                }

                if self.can_castle_kingside(position) {
                    moves.push(Move {
                        from: position,
                        to: (position.0, 6),
                        castling: true,
                    })
                }
                if self.can_castle_queenside(position) {
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

    /// Analyzes the board to tell if the king at the given position can castle kingside
    ///
    /// The rules for castling are:
    ///
    /// - Neither the king nor the chosen rook has previously moved during the game.
    /// - There are no pieces between the king and the chosen rook.
    /// - The king is not currently in check.
    /// - The squares that the king passes over are not attacked by an enemy piece, nor is the square where the king lands.
    /// - The king does not pass through a square that is attacked by an enemy piece.
    pub fn can_castle_kingside(&self, king_position: (usize, usize)) -> bool {
        let (x, y) = king_position;
        if y != 4 {
            return false;
        }

        if let (Some(king), Some(rook)) = (self.squares[x][y].piece, self.squares[x][7].piece) {
            let targeted = self.targeted_squares(king.color.inverse());
            // Ensure that the pieces are the right type ...
            return king.piece_type == PieceType::King && rook.piece_type == Rook
                // .. and color
                && rook.color == king.color
                // ... the king and the kingside rook haven't moved
                && king.moves == 0 && rook.moves == 0
                // ... the squares between them are empty,
                && self.squares[x][6].is_empty()
                && self.squares[x][5].is_empty()
                // ... the king isn't in check,
                && !targeted.contains(&king_position)
                //  ... doesn't move through check,
                && !targeted.contains(&(x, 5))
                // ... and isn't castling into check.
                && !targeted.contains(&(x, 6));
        }
        false
    }

    /// Analyzes the board to tell if the king at the given position can castle kingside
    pub fn can_castle_queenside(&self, king_position: (usize, usize)) -> bool {
        let (x, y) = king_position;
        if y != 4 {
            return false;
        }

        if let (Some(king), Some(rook)) = (self.squares[x][y].piece, self.squares[x][0].piece) {
            let targeted = self.targeted_squares(king.color.inverse());
            // Ensure that the pieces are the right type ...
            return king.piece_type == PieceType::King && rook.piece_type == Rook
                // .. and color
                && rook.color == king.color
                // ... the king and the kingside rook haven't moved
                && king.moves == 0 && rook.moves == 0
                // ... the squares between them are empty,
                && self.squares[x][1].is_empty()
                && self.squares[x][2].is_empty()
                && self.squares[x][3].is_empty()
                // ... the king isn't in check,
                && !targeted.contains(&king_position)
                //  ... doesn't move through check,
                && !targeted.contains(&(x, 1))
                && !targeted.contains(&(x, 3))
                // ... and isn't castling into check.
                && !targeted.contains(&(x, 2));
        }
        false
    }

    /// Moves the piece and increments the movements counter
    pub fn move_piece(&mut self, mov: Move) -> Result<String, ChessMoveError> {
        let (from_x, from_y) = mov.from;
        let (to_x, to_y) = mov.to;

        // Verify the move is within the bounds of the board
        if from_x >= BOARD_SIZE || from_y >= BOARD_SIZE || to_x >= BOARD_SIZE || to_y >= BOARD_SIZE {
            return Err(ChessMoveError::OutOfBounds);
        }


        // Ensure the start piece is not missing
        if self.squares[from_x][from_y].is_empty() {
            return Err(ChessMoveError::StartPieceMissing);
        }

        // Do we have capture?
        let action_str = if !self.squares[to_x][to_y].is_empty() {
            format!("{:?} at ({}, {}) captures {:?} at ({}, {})",
                    self.squares[from_x][from_y].piece.unwrap().piece_type, from_x, from_y,
                    self.squares[to_x][to_y].piece.unwrap().piece_type, to_x, to_y
            )
        } else {
            format!("{:?} at ({}, {}) moves to ({}, {})",
                    self.squares[from_x][from_y].piece.unwrap().piece_type, from_x, from_y,
                    to_x, to_y
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
        self.highlighted.push(self.squares[from_x][from_y]);
        self.highlighted.push(self.squares[to_x][to_y]);

        Ok(action_str)
    }

    /// Performs castling, constrained by the rules described on `can_castle_kingside`
    pub fn castle(&mut self, color: Color, castle_type: ChessMove) -> Result<String, ChessMoveError> {
        let row = match color {
            White => { 7 }
            Black => { 0 }
        };

        let king_col = 4;
        let nw_king_col: usize;
        let rook_col: usize;
        let nw_rook_col: usize;

        match castle_type {
            ChessMove::CastleKingside => {
                nw_king_col = 6;
                rook_col = 7;
                nw_rook_col = 5;
            },
            ChessMove::CastleQueenside => {
                nw_king_col = 2;
                rook_col = 0;
                nw_rook_col = 3;
            },
            _ => { return Err(ChessMoveError::CastlingForbidden); }
        };

        if self.can_castle_kingside((row, king_col)) {
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
            self.highlighted.push(self.squares[row][king_col]);
            self.highlighted.push(self.squares[row][nw_king_col]);
            self.highlighted.push(self.squares[row][rook_col]);
            self.highlighted.push(self.squares[row][nw_rook_col]);
            return match castle_type {
                ChessMove::CastleKingside => Ok("Castles kingside".parse().unwrap()),
                ChessMove::CastleQueenside => Ok("Castles Queenside".parse().unwrap()),
                _ => Err(ChessMoveError::CastlingForbidden)
            };
        }

        Err(ChessMoveError::CastlingForbidden)
    }
}


impl FromStr for ChessBoard {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        if s.is_empty() { return Err(EmptyString); }
        fn from_fen_str(representation: &str) -> Result<ChessBoard, ParseError> {
            let re: Regex = RegexBuilder::new(FEN_REGEX)
                .case_insensitive(true)
                .build()
                .unwrap();

            if !re.is_match(representation) {
                return Err(ParseError::InvalidFENString);
            }
            let caps = re.captures(representation).unwrap();
            let pieces = &caps["board"];
            let turn = caps.name("turn").unwrap().as_str();
            // let castling = caps.name("castling").unwrap().as_str();
            let passant = caps.name("passant").unwrap().as_str();
            let halfmove = &caps["halfmove"].parse::<u32>().unwrap();
            let fullmove = &caps["fullmove"].parse::<u32>().unwrap();

            let mut rank: usize = BOARD_SIZE;
            let mut file_iter = FILE_CHAR_RANGE.cycle();
            let mut board = ChessBoard::new();


            // Load the pieces
            for c in pieces.chars() {
                let file = &file_iter.next().clone().unwrap();
                match c {
                    'K' => {
                        board.set_piece(rank, file, Color::White, PieceType::King);
                    }
                    'Q' => {
                        board.set_piece(rank, file, Color::White, PieceType::Queen);
                    }
                    'R' => {
                        board.set_piece(rank, file, Color::White, PieceType::Rook);
                    }
                    'B' => {
                        board.set_piece(rank, file, Color::White, PieceType::Bishop);
                    }
                    'N' => {
                        board.set_piece(rank, file, Color::White, PieceType::Knight);
                    }
                    'P' => {
                        board.set_piece(rank, file, Color::White, PieceType::Pawn);
                    }
                    'k' => {
                        board.set_piece(rank, file, Color::Black, PieceType::King);
                    }
                    'q' => {
                        board.set_piece(rank, file, Color::Black, PieceType::Queen);
                    }
                    'r' => {
                        board.set_piece(rank, file, Color::Black, PieceType::Rook);
                    }
                    'b' => {
                        board.set_piece(rank, file, Color::Black, PieceType::Bishop);
                    }
                    'n' => {
                        board.set_piece(rank, file, Color::Black, PieceType::Knight);
                    }
                    'p' => {
                        board.set_piece(rank, file, Color::Black, PieceType::Pawn);
                    }
                    '1'..='8' => {
                        // rank += c.to_digit(10).unwrap() as usize;
                        for _ in file_iter.by_ref().take(c.to_digit(10).unwrap() as usize - 1) {}
                    }
                    '/' => {
                        rank -= 1;
                        file_iter = FILE_CHAR_RANGE.cycle();
                    }
                    _ => {}
                }
            }

            // Set the turn
            board.active_color = Color::from_str(turn).unwrap();

            // Castling
            // TODO: Implement castling loading

            // En passant
            // TODO: Implement the rest of en passant rules

            board.passant_square = match passant {
                "-" => None,
                _ => {
                    let p_file = passant.as_bytes()[0] as char;
                    let p_rank = passant.as_bytes()[1] as usize;
                    Some(board.squares[rank_to_index(p_rank)][c_file_to_index(&p_file)])
                }
            };
            board.half_moves = *halfmove;
            board.full_moves = *fullmove;

            Ok(board)
        }

        from_fen_str(s)
    }
}