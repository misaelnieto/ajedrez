#[cfg(test)]
mod tests {
    use ajedrez::{ChessBoard, Color, PieceType};

    #[test]
    fn test_board_fen_code() {
        let mut board = ChessBoard::new();
        // Set up the pieces on the chessboard
        board.set_piece(8, &'a', Color::White, PieceType::Rook);
        board.set_piece(8, &'h', Color::White, PieceType::Rook);
        board.set_piece(1, &'a', Color::Black, PieceType::Rook);
        board.set_piece(1, &'h', Color::Black, PieceType::Rook);

        assert_eq!("R6R/8/8/8/8/8/8/r6r w - - 0 0", board.as_fen());
    }
}