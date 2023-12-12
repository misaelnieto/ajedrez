#[cfg(test)]
mod tests {
    use crate::{ChessBoard, Color, PieceType};

    #[test]
    fn test_board_fen_code() {
        let mut board = ChessBoard::new();
        // Set up the pieces on the chessboard
        board.set_piece(0, 0, Color::White, PieceType::Rook);
        board.set_piece(0, 7, Color::White, PieceType::Rook);
        board.set_piece(7, 0, Color::Black, PieceType::Rook);
        board.set_piece(7, 7, Color::Black, PieceType::Rook);

        assert_eq!("R6R/8/8/8/8/8/8/r6r w - - 0 0", board.as_fen());
    }



}