mod tests;

use std::fmt;

const BOARD_SIZE: usize = 8; // Chessboard is 8x8

// Define an enum for the color of the pieces
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
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

// Define a struct that will represent a chess piece with a color and type
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

// For convenience, we can implement a constructor for the Piece struct
impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Piece { color, piece_type }
    }

    pub fn as_fen(&self) -> char {
        match self.color {
            Color::White => {
                match self.piece_type {
                    PieceType::Pawn => 'P',
                    PieceType::Knight => 'K',
                    PieceType::Bishop => 'B',
                    PieceType::Rook => 'R',
                    PieceType::Queen => 'Q',
                    PieceType::King => 'K',
                }
            },
            Color::Black => {
                match self.piece_type {
                    PieceType::Pawn => 'p',
                    PieceType::Knight => 'k',
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
            },
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

pub struct ChessBoard {
    squares: [[Square; BOARD_SIZE]; BOARD_SIZE],
}


impl ChessBoard {
    pub fn new() -> Self {
        let empty_square = Square::new(None);
        let squares = [[empty_square; BOARD_SIZE]; BOARD_SIZE]; // Initialize all squares to empty
        ChessBoard { squares }
    }

    pub fn set_piece(&mut self, rank: usize, file: usize, piece: Piece) {
        if rank < BOARD_SIZE && file < BOARD_SIZE {
            self.squares[rank][file] = Square::new(Some(piece));
        }
    }

    pub fn get_piece(&self, rank: usize, file: usize) -> Option<Piece> {
        if rank < BOARD_SIZE && file < BOARD_SIZE {
            self.squares[rank][file].0
        } else {
            None
        }
    }

    pub fn print_board(&self) {
        print!("╭{}╮\n", "─".repeat(BOARD_SIZE * 4 -1));
        for rank in 0..BOARD_SIZE {
            print!("│");
            for file in 0..BOARD_SIZE {
                match self.squares[rank][file].0 {
                    Some(piece) => print!(" {} │", piece),
                    None => print!("   │"),
                }
            }
            print!("\n├{}┤\n", "─".repeat(BOARD_SIZE * 4 -1));
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
                    continue
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
            fen_code.push('/');
        }
        fen_code
    }
}
