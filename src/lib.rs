use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;

use regex::{Regex, RegexBuilder};

use crate::ParseChessBoardError::EmptyString;

mod tests;

const BOARD_SIZE: usize = 8; // Chessboard is 8x8


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
/// # Example:
///
/// To create a single piece
///
/// ```
/// let mut p  = Piece::new(Color::White, PieceType::King, 0, 0);
/// assert_eq!('K', p.as_fen());
/// ```
///
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
    pub rank: usize,
    pub file: char,
    pub moves: u32,
}

// For convenience, we can implement a constructor for the Piece struct
impl Piece {
    pub fn new(color: Color, piece_type: PieceType, rank: usize, file: char) -> Self {
        Piece { color, piece_type, rank, file, moves: 0 }
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

    /*
        The piece's board coordinate in algebraic notation
     */
    pub fn board_coordinate(&self) -> String {
        format!("{}{}", self.rank, self.file)
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

#[derive(Debug, Clone, Copy)]
pub struct Square(Option<Piece>);

impl Square {
    pub fn new(piece: Option<Piece>) -> Self {
        Square(piece)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn as_fen(&self) -> char {
        self.0.unwrap().as_fen()
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
    active_color: Color,
    /// This is a counter of the halfmoves (or ply) since the last capture or pawn advance. This
    /// field is used for the fifty-move rule, which allows a player to claim a draw if no capture
    /// or pawn move has occurred in the last fifty moves.
    half_moves: u32,
    full_moves: u32,
    passant_square: Option<Square>,
}

const RANK_RANGE: RangeInclusive<usize> = 1..=BOARD_SIZE;
const FILE_RANGE: RangeInclusive<char> = 'a'..='h';

fn file_to_usize(file: &char) -> usize {
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


impl ChessBoard {
    pub fn new() -> Self {
        let empty_square = Square::new(None);
        let squares = [[empty_square; BOARD_SIZE]; BOARD_SIZE]; // Initialize all squares to empty
        ChessBoard { squares, active_color: Color::White, full_moves: 0, half_moves: 0, passant_square: None }
    }

    pub fn set_piece(&mut self, rank: usize, file: &char, color: Color, piece_type: PieceType) -> &mut ChessBoard {
        if !RANK_RANGE.contains(&rank) {
            panic!("Rank must be a number between 1 and 8, you provided {rank}")
        }
        if !FILE_RANGE.contains(&file) {
            panic!("Rank must be a letter between a and h, you provided {file}")
        }
        self.squares[BOARD_SIZE - rank][file_to_usize(file)] = Square::new(Some(Piece::new(color, piece_type, rank, file.clone())));
        self
    }

    pub fn get_piece(&self, rank: usize, file: &char) -> Option<Piece> {
        if !RANK_RANGE.contains(&rank) {
            panic!("Rank must be a number between 1 and 8, you provided {rank}")
        }
        if !FILE_RANGE.contains(&file) {
            panic!("File must be a letter between a and h, you provided {file}")
        }
        self.squares[BOARD_SIZE - rank][file_to_usize(file)].0
    }

    /// Active Color: The next field indicates whose turn it is to move. "w" means it is White's
    /// turn, and "b" means it is Black's turn.
    pub fn active_color(&self) -> Color {
        self.active_color
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

    /*
        En Passant Target Square: If there's a square where an en passant capture is possible, that
        square is noted here. It's recorded using algebraic notation (e.g., "e3").
        If there's no en passant target square, this is represented by a dash "-".
    */
    pub fn passant_square(&self) -> String {
        if self.passant_square.is_none() {
            return '-'.to_string();
        }
        self.passant_square.unwrap().0.unwrap().board_coordinate()
    }

    /// Halfmove Clock: This is a counter of the halfmoves (or ply) since the last capture or pawn
    /// advance. This field is used for the fifty-move rule, which allows a player to claim a draw
    /// if no capture or pawn move has occurred in the last fifty moves.
    pub fn half_moves(&self) -> u32 { self.half_moves }

    /// Fullmove Counter: This is a counter of the full moves in the game. It starts at 1 and
    /// Fullmove Counter: This is a counter of the full moves in the game. It starts at 1 and
    /// increments after Black's move.
    pub fn full_moves(&self) -> u32 { self.full_moves }

    pub fn print_board(&self) {
        print!("╭{}╮\n", "─".repeat(BOARD_SIZE * 4 - 1));
        for rank in 0..BOARD_SIZE {
            print!("│");
            for file in 0..BOARD_SIZE {
                match self.squares[rank][file].0 {
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
                      self.passant_square(),
                      self.half_moves(),
                      self.full_moves())
        );
        fen_code
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum ParseChessBoardError{
    EmptyString,
    InvalidFENString,
}

impl FromStr for ChessBoard {
    type Err = ParseChessBoardError;
    fn from_str(s: &str) -> Result<Self, ParseChessBoardError> {
        if s.is_empty() { return Err(EmptyString); }
        fn from_fen_str(representation: &str) -> Result<ChessBoard, ParseChessBoardError> {
            let re: Regex = RegexBuilder::new(FEN_REGEX)
                .case_insensitive(true)
                .build()
                .unwrap();

            if !re.is_match(representation) {
                return Err(ParseChessBoardError::InvalidFENString);
            }
            let caps = re.captures(representation).unwrap();
            let pieces = &caps["board"];
            let turn = caps.name("turn").unwrap().as_str();
            // let castling = caps.name("castling").unwrap().as_str();
            let passant = caps.name("passant").unwrap().as_str();
            let halfmove = &caps["halfmove"].parse::<u32>().unwrap();
            let fullmove = &caps["fullmove"].parse::<u32>().unwrap();

            let mut rank: usize = BOARD_SIZE;
            let mut file_iter = FILE_RANGE.cycle();
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
                        rank -=1;
                        file_iter = FILE_RANGE.cycle();
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
            if passant == "-" {
                board.passant_square = None;
            }
            board.half_moves = *halfmove;
            board.full_moves = *fullmove;

            Ok(board)
        }

        from_fen_str(s)
    }
}