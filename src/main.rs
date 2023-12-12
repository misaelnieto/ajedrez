use ajedrez::{ChessBoard, Color, PieceType};

fn main() {
    let mut board = ChessBoard::new();

    // Set up the pieces on the chessboard
    board.set_piece(8, &'a', Color::White, PieceType::Rook);
    board.set_piece(8, &'b', Color::White, PieceType::Rook);
    board.set_piece(1, &'b', Color::Black, PieceType::Rook);
    board.set_piece(1, &'c', Color::Black, PieceType::Rook);

    // Example of printing the board state to the console
    board.print_board();
}
