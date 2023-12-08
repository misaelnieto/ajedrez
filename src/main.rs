use ajedrez::{ChessBoard, Color, Piece, PieceType};

fn main() {
    let mut board = ChessBoard::new();

    // Set up the pieces on the chessboard
    board.set_piece(0, 0, Piece::new(Color::White, PieceType::Rook));
    board.set_piece(0, 7, Piece::new(Color::White, PieceType::Rook));
    board.set_piece(7, 0, Piece::new(Color::Black, PieceType::Rook));
    board.set_piece(7, 7, Piece::new(Color::Black, PieceType::Rook));

    // Example of printing the board state to the console
    board.print_board();
}
