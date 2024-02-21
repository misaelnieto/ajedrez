#[cfg(test)]
mod tests {
    use ajedrez::Color::{Black, White};
    use ajedrez::PieceType::{King, Rook};
    use ajedrez::{
        ChessBoard, Color, Piece, PieceType, Square, DEFAULT_KINGSIDE_ROOK_COL, DEFAULT_KING_COL,
        DEFAULT_QUEENSIDE_ROOK_COL,
    };

    #[test]
    fn test_square_as_board_coordinate() {
        let mut sq = Square::default();
        assert_eq!("-", format!("{sq}"));
        sq = Square {
            piece: Option::from(Piece::new(Color::White, PieceType::King)),
            rank: 8,
            file: 'a',
            row: 0,
            col: 0,
        };
        assert_eq!("8a", format!("{sq}"));
        sq = Square {
            piece: Option::from(Piece::new(Color::White, PieceType::King)),
            rank: 2,
            file: 'c',
            row: 6,
            col: 2,
        };
        assert_eq!("2c", format!("{sq}"));
    }

    #[test]
    fn test_castling() {
        let mut board = ChessBoard::new();
        let castling = board
            .set_piece_0(
                0,
                DEFAULT_QUEENSIDE_ROOK_COL,
                Some(Piece {
                    color: Black,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .set_piece_0(
                0,
                DEFAULT_KING_COL,
                Some(Piece {
                    color: Black,
                    piece_type: King,
                    moves: 0,
                }),
            )
            .set_piece_0(
                0,
                DEFAULT_KINGSIDE_ROOK_COL,
                Some(Piece {
                    color: Black,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .set_piece_0(
                7,
                DEFAULT_QUEENSIDE_ROOK_COL,
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .set_piece_0(
                7,
                DEFAULT_KING_COL,
                Some(Piece {
                    color: White,
                    piece_type: King,
                    moves: 0,
                }),
            )
            .set_piece_0(
                7,
                DEFAULT_KINGSIDE_ROOK_COL,
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .get_castling(true);

        assert_eq!(castling.white_kingside, true);
        assert_eq!(castling.white_queenside, true);
        assert_eq!(castling.black_kingside, true);
        assert_eq!(castling.black_queenside, true);
        assert_eq!(castling.check_empty_rows, true);
    }

    #[test]
    fn test_castling_white() {
        let mut board = ChessBoard::new();
        let castling = board
            .set_piece_0(
                7,
                DEFAULT_QUEENSIDE_ROOK_COL,
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .set_piece_0(
                7,
                DEFAULT_KING_COL,
                Some(Piece {
                    color: White,
                    piece_type: King,
                    moves: 0,
                }),
            )
            .set_piece_0(
                7,
                DEFAULT_KINGSIDE_ROOK_COL,
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .get_castling(true);

        assert_eq!(castling.white_kingside, true);
        assert_eq!(castling.white_queenside, true);
        assert_eq!(castling.black_kingside, false);
        assert_eq!(castling.black_queenside, false);
    }

    #[test]
    fn test_castling_white_piece_moved() {
        let mut board = ChessBoard::new();
        let mut castling = board
            .set_piece_0(
                7,
                DEFAULT_KINGSIDE_ROOK_COL,
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                    moves: 1,
                }),
            )
            .set_piece_0(
                7,
                DEFAULT_QUEENSIDE_ROOK_COL,
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                    moves: 1,
                }),
            )
            .set_piece_0(
                7,
                DEFAULT_KING_COL,
                Some(Piece {
                    color: White,
                    piece_type: King,
                    moves: 1,
                }),
            )
            .get_castling(true);

        assert_eq!(castling.white_kingside, false);
        assert_eq!(castling.white_queenside, false);

        // The king hasn't moved
        castling = board
            .set_piece_0(
                7,
                DEFAULT_KING_COL,
                Some(Piece {
                    color: White,
                    piece_type: King,
                    moves: 0,
                }),
            )
            .get_castling(true);
        assert_eq!(castling.white_kingside, false);
        assert_eq!(castling.white_queenside, false);

        // Kingside Rook didn't move
        castling = board
            .set_piece_0(
                7,
                DEFAULT_KINGSIDE_ROOK_COL,
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .get_castling(true);
        assert_eq!(castling.white_kingside, true);
        assert_eq!(castling.white_queenside, false);

        // Qeeenside Rook didn't move
        castling = board
            .set_piece_0(
                7,
                DEFAULT_QUEENSIDE_ROOK_COL,
                Some(Piece {
                    color: White,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .get_castling(true);
        assert_eq!(castling.white_kingside, true);
        assert_eq!(castling.white_queenside, true);

        // The king moved, but the rooks didn't
        castling = board
            .set_piece_0(
                7,
                DEFAULT_KING_COL,
                Some(Piece {
                    color: White,
                    piece_type: King,
                    moves: 1,
                }),
            )
            .get_castling(true);
        assert_eq!(castling.white_kingside, false);
        assert_eq!(castling.white_queenside, false);
    }

    #[test]
    fn test_castling_black() {
        let mut board = ChessBoard::new();
        let castling = board
            .set_piece_0(
                0,
                DEFAULT_QUEENSIDE_ROOK_COL,
                Some(Piece {
                    color: Black,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .set_piece_0(
                0,
                DEFAULT_KING_COL,
                Some(Piece {
                    color: Black,
                    piece_type: King,
                    moves: 0,
                }),
            )
            .set_piece_0(
                0,
                DEFAULT_KINGSIDE_ROOK_COL,
                Some(Piece {
                    color: Black,
                    piece_type: Rook,
                    moves: 0,
                }),
            )
            .get_castling(true);

        assert_eq!(castling.white_kingside, false);
        assert_eq!(castling.white_queenside, false);
        assert_eq!(castling.black_kingside, true);
        assert_eq!(castling.black_queenside, true);
        assert_eq!(castling.check_empty_rows, true);
    }
}
