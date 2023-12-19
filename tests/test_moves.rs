#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ajedrez::{ChessBoard, Move, ParseError};

    #[test]
    fn test_move_from_str() {
        // Invalid strings
        assert_eq!(Err(ParseError::StringTooShort), Move::from_str("x"));
        assert_eq!(Err(ParseError::StringTooShort), Move::from_str("123"));
        assert_eq!(Err(ParseError::InvalidPositionFile), Move::from_str("i1i1"));
        assert_eq!(Err(ParseError::InvalidPositionFile), Move::from_str("01i1"));
        assert_eq!(Err(ParseError::InvalidPositionRank), Move::from_str("a0a0"));
        assert_eq!(Err(ParseError::InvalidPositionRank), Move::from_str("aza0"));
        assert_eq!(Err(ParseError::InvalidPositionRank), Move::from_str("a1h9"));
        assert_eq!(Err(ParseError::UselessMove), Move::from_str("a1a1"));

        let mut mov = Move::from_str("a8b8").expect("parsing move should not have failed");
        assert_eq!((0,0), mov.from);
        assert_eq!((0,1), mov.to);
        mov = Move::from_str("a1a8").expect("parsing move should not have failed");
        assert_eq!((7,0), mov.from);
        assert_eq!((0,0), mov.to);
    }

    #[test]
    fn test_generate_pawn_moves_base() {
        let board = ChessBoard::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();

        // Fail gracefully: empty moves for anything else but pawns
        for i in [0,2,3,4,5,7] {
            for j in 0..=7 {
                let possible_moves = board.generate_pawn_moves((i, j));
                assert_eq!(0, possible_moves.len());
            }
        }

        // On first move, every pawn has two options
        for i in 0..=7 {
            let mut possible_moves = board.generate_pawn_moves((1, i));
            assert_eq!(2, possible_moves.len());
            assert_eq!((1, i), possible_moves[0].from);
            assert_eq!((2, i), possible_moves[0].to);
            assert_eq!((1, i), possible_moves[1].from);
            assert_eq!((3, i), possible_moves[1].to);

            possible_moves = board.generate_pawn_moves((6, i));
            assert_eq!(2, possible_moves.len());
            assert_eq!((6, i), possible_moves[0].from);
            assert_eq!((5, i), possible_moves[0].to);
            assert_eq!((6, i), possible_moves[1].from);
            assert_eq!((4, i), possible_moves[1].to);
        }
    }

    #[test]
    fn test_generate_pawn_moves_capture() {
        // White pawn
        let mut board = ChessBoard::from_str("8/8/2rbk3/3P4/8/8/8/8 w - - 0 0").unwrap();
        // The pawn may capture either the rook or the knight, but not the bishop, which blocks
        // the pawn from moving directly forward. So only 2 moves
        let possible_moves = board.generate_pawn_moves((3, 3));
        assert_eq!(2, possible_moves.len());
        // The pawn may capture either the rook ...
        assert_eq!((2, 2), possible_moves[0].to);
        // or the Knight
        assert_eq!((2, 4), possible_moves[1].to);

        // Black pawn
        board = ChessBoard::from_str("8/8/8/8/3p4/2RBK3/8/8 w - - 0 0").unwrap();
        let possible_moves = board.generate_pawn_moves((4, 3));
        assert_eq!(2, possible_moves.len());
        assert_eq!((5, 2), possible_moves[0].to);
        assert_eq!((5, 4), possible_moves[1].to);
    }
}
