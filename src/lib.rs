use std::fmt;
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

use regex::{Regex, RegexBuilder};

use crate::ParseError::EmptyString;

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
}


/// Converts a chess rank to a zero-based index
pub fn rank_to_index(rank: usize) -> usize {
    match BOARD_SIZE_RANGE_1.contains(&rank) {
        true => BOARD_SIZE - rank,
        false => panic!("Rank must be a number between 1 and 8, you provided {rank}")
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


/// Converts a chess file to a zero-based index
fn file_to_index(file: &char) -> usize {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChessMoveError {
    OutOfBounds,
    StartPieceMissing,
    NotImplemented,
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
        ChessBoard { squares, active_color: Color::White, full_moves: 0, half_moves: 0, passant_square: None }
    }

    pub fn set_piece(&mut self, rank: usize, file: &char, color: Color, piece_type: PieceType) -> &mut ChessBoard {
        let index_rank = rank_to_index(rank);
        let index_file = file_to_index(file);
        self.squares[index_rank][index_file].piece = Some(Piece::new(color, piece_type));
        self
    }

    pub fn get_piece(&self, rank: usize, file: &char) -> Option<Piece> {
        self.squares[rank_to_index(rank)][file_to_index(file)].piece
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

    pub fn print_board(&self) {
        print!("╭{}╮\n", "─".repeat(BOARD_SIZE * 4 - 1));
        for rank in 0..BOARD_SIZE {
            print!("│");
            for file in 0..BOARD_SIZE {
                match self.squares[rank][file].piece {
                    Some(piece) => print!(" {} │", piece),
                    None => print!("   │"),
                }
            }
            print!("\n├{}┤\n", "─".repeat(BOARD_SIZE * 4 - 1));
        }
        print!("╰{}╯\n", "─".repeat(BOARD_SIZE * 4 - 1));
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
    pub fn generate_pawn_moves(&self, position: (usize, usize)) -> Vec<Move> {
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
                moves.push(Move { from: position, to: (fwd, y) });
            }

            // Capture diagonally, to the left, except for first file/column
            if (y as isize - 1) > 0 {
                let left = y - 1;
                if BOARD_SIZE_RANGE_0.contains(&left) && !self.squares[fwd][left].is_empty() {
                    moves.push(Move { from: position, to: (fwd, left) });
                }
            }

            // Capture diagonally, to the right, except for last file/column
            let right = y + 1;
            if BOARD_SIZE_RANGE_0.contains(&right) && !self.squares[fwd][right].is_empty() {
                moves.push(Move { from: position, to: (fwd, right) });
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
                moves.push(Move { from: position, to: (fwd, y) });
            }
        }

        // TODO: en passant
        moves
    }

    pub fn generate_knight_moves(&self, position: (usize, usize)) -> Vec<Move> {
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
                moves.push(Move { from: position, to: (new_x as usize, new_y as usize) });
            }
        }

        moves
    }


    pub fn generate_bishop_moves(&self, position: (usize, usize)) -> Vec<Move> {
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
                            });
                        }
                        break; // Stop moving in this direction whether a piece was captured or it's blocked
                    }
                    None => {
                        // If the square is empty, it's a valid move
                        moves.push(Move {
                            from: position,
                            to: current_position,
                        });
                    }
                }
            }
        }

        moves
    }


    pub fn generate_rook_moves(&self, position: (usize, usize)) -> Vec<Move> {
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
                            moves.push(Move { from: position, to: (x, y) });
                        }
                        // Whether it's a capture or not, the rook can't move past this piece
                        break;
                    }
                    None => {
                        // Add the move to the list if the square is empty
                        moves.push(Move { from: position, to: (x, y) });
                    }
                }
            }
        }

        moves
    }

    pub fn generate_king_moves(&self, position: (usize, usize)) -> Vec<Move> {
        let mut moves = Vec::new();
        let king = match self.squares[position.0][position.1].piece {
            Some(p) => p,
            None => return moves, // No king, so no moves.
        };

        // Ensure that the piece is a king
        if king.piece_type != PieceType::King {
            return moves; // Not a king, so no moves.
        }
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
                            moves.push(Move { from: position, to: (new_x as usize, new_y as usize) });
                        }
                        // Otherwise, the king cannot move into a square occupied by an allied piece
                    }
                    None => {
                        // If the square is unoccupied, it's a valid move
                        moves.push(Move { from: position, to: (new_x as usize, new_y as usize) });
                    }
                }
            }
        }

        // Castling could be added here if you are checking board state, i.e., whether rooks and king
        // have moved, and if there is a clear path for castling.

        moves
    }

    pub fn generate_queen_moves(&self, position: (usize, usize)) -> Vec<Move> {
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
                            moves.push(Move { from: position, to: (x, y) });
                        }
                        // Since a piece is on this square, the queen cannot move past; break the loop
                        break;
                    }
                    None => {
                        // No piece on the square, the queen can move here
                        moves.push(Move { from: position, to: (x, y) });
                    }
                }
            }
        }

        moves
    }

    pub fn generate_moves(&self, position: (usize, usize)) -> Result<Vec<Move>, ChessMoveError> {
        if self.squares[position.0][position.1].is_empty() {
            return Err(ChessMoveError::StartPieceMissing);
        }
        match self.squares[position.0][position.1].piece.unwrap().piece_type {
            PieceType::Pawn => Ok(self.generate_pawn_moves(position)),
            PieceType::Knight => Ok(self.generate_knight_moves(position)),
            PieceType::Bishop => Ok(self.generate_bishop_moves(position)),
            PieceType::Rook => Ok(self.generate_rook_moves(position)),
            PieceType::King => Ok(self.generate_king_moves(position)),
            PieceType::Queen => Ok(self.generate_queen_moves(position)),
        }
    }

    pub fn move_piece(&mut self, mov: Move) -> Result<(), ChessMoveError> {
        let (from_x, from_y) = mov.from;
        let (to_x, to_y) = mov.to;

        // Verify the move is within the bounds of the board
        if from_x >= BOARD_SIZE || from_y >= BOARD_SIZE || to_x >= BOARD_SIZE || to_y >= BOARD_SIZE {
            return Err(ChessMoveError::OutOfBounds);
        }

        // Retrieve the piece from the starting square
        let piece = match self.squares[from_x][from_y].piece {
            Some(piece) => piece,
            None => return Err(ChessMoveError::StartPieceMissing),
        };

        // Remove the piece from the starting square
        self.squares[from_x][from_y].piece = None;
        self.squares[from_x][from_y].piece = Some(piece);

        Ok(())
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
                    Some(board.squares[rank_to_index(p_rank)][file_to_index(&p_file)])
                }
            };
            board.half_moves = *halfmove;
            board.full_moves = *fullmove;

            Ok(board)
        }

        from_fen_str(s)
    }
}