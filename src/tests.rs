#[cfg(test)]
mod tests {
    use crate::{ChessBoard, Color, Piece, PieceType};

    #[test]
    fn test_board_fen_code() {
        let mut board = ChessBoard::new();
        // Set up the pieces on the chessboard
        board.set_piece(0, 0, Piece::new(Color::White, PieceType::Rook));
        board.set_piece(0, 7, Piece::new(Color::White, PieceType::Rook));
        board.set_piece(7, 0, Piece::new(Color::Black, PieceType::Rook));
        board.set_piece(7, 7, Piece::new(Color::Black, PieceType::Rook));

        assert_eq!("R6R/8/8/8/8/8/8/r6r/", board.as_fen());
    }
}