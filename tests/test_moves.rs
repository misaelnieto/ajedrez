#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ajedrez::{ChessBoard, Move, ParseError, BOARD_SIZE_RANGE_0};

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

    #[test]
    fn test_generate_knight_moves_base() {
        let board = ChessBoard::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();
        let default_knight_positions = [(0,1), (0,6), (7,1), (7,6)];

        // Fail gracefully: empty moves for anything else but Knights
        for i in BOARD_SIZE_RANGE_0 {
            for j in BOARD_SIZE_RANGE_0 {
                if default_knight_positions.contains(&(i,j)) {
                    continue
                }
                let possible_moves = board.generate_knight_moves((i, j));
                assert_eq!(0, possible_moves.len());
            }
        }

        // Default movements for Knight at b8
        let pos = default_knight_positions[0];
        let possible_moves = board.generate_knight_moves(pos);
        assert_eq!(2, possible_moves.len());
        assert_eq!((2,2), possible_moves[0].to);
        assert_eq!((2,0), possible_moves[1].to);

        // Default movements for Knight at g8
        let pos = default_knight_positions[1];
        let possible_moves = board.generate_knight_moves(pos);
        assert_eq!(2, possible_moves.len());
        assert_eq!((2,7), possible_moves[0].to);
        assert_eq!((2,5), possible_moves[1].to);

        // Default movements for Knight at b1
        let pos = default_knight_positions[2];
        let possible_moves = board.generate_knight_moves(pos);
        assert_eq!(2, possible_moves.len());
        assert_eq!((5,2), possible_moves[0].to);
        assert_eq!((5,0), possible_moves[1].to);

        // Default movements for Knight at b8
        let pos = default_knight_positions[3];
        let possible_moves = board.generate_knight_moves(pos);
        assert_eq!(2, possible_moves.len());
        assert_eq!((5,7), possible_moves[0].to);
        assert_eq!((5,5), possible_moves[1].to);
    }

    #[test]
    fn test_generate_knight_moves_no_pieces() {
        let board = ChessBoard::from_str("1n6/8/8/8/8/5N2/8/8 b KQkq - 0 1").unwrap();

        // Black horse only has 3 possible moves ...
        let mut possible_moves = board.generate_knight_moves((0, 1));
        assert_eq!(3, possible_moves.len());
        assert_eq!((1,3), possible_moves[0].to);
        assert_eq!((2,2), possible_moves[1].to);
        assert_eq!((2,0), possible_moves[2].to);


        possible_moves = board.generate_knight_moves((5, 5));
        // ... while the White horse has all 8 possible moves
        assert_eq!(8, possible_moves.len());
        assert_eq!((6,7), possible_moves[0].to);
        assert_eq!((7,6), possible_moves[1].to);
        assert_eq!((4,7), possible_moves[2].to);
        assert_eq!((3,6), possible_moves[3].to);
        assert_eq!((6,3), possible_moves[4].to);
        assert_eq!((7,4), possible_moves[5].to);
        assert_eq!((4,3), possible_moves[6].to);
        assert_eq!((3,4), possible_moves[7].to);
    }

    #[test]
    fn test_generate_bishop_moves_base() {
        let board = ChessBoard::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap();

        // Fail gracefully: On this initial setup, bishops cannot move at all
        for i in BOARD_SIZE_RANGE_0 {
            for j in BOARD_SIZE_RANGE_0 {
                let possible_moves = board.generate_bishop_moves((i, j));
                assert_eq!(0, possible_moves.len());
            }
        }
    }

    #[test]
    fn test_generate_bishop_moves_initial_empty() {
        let board = ChessBoard::from_str("2b2b2/8/8/8/8/8/8/2B2B2 w - - 0 0").unwrap();
        let default_bishop_positions = [(0, 2), (0, 5), (7, 2), (7, 5)];

        // Default movements for Knight at c8
        let mut possible_moves = board.generate_bishop_moves(default_bishop_positions[0]);
        assert_eq!(7, possible_moves.len());
        assert_eq!((1, 1), possible_moves[0].to);
        assert_eq!((2, 0), possible_moves[1].to);
        assert_eq!((2, 4), possible_moves[3].to);
        assert_eq!((3, 5), possible_moves[4].to);
        assert_eq!((4, 6), possible_moves[5].to);
        assert_eq!((5, 7), possible_moves[6].to);

        // Default movements for Knight at f8
        possible_moves = board.generate_bishop_moves(default_bishop_positions[1]);
        assert_eq!(7, possible_moves.len());
        assert_eq!((1, 4), possible_moves[0].to);
        assert_eq!((2, 3), possible_moves[1].to);
        assert_eq!((4, 1), possible_moves[3].to);
        assert_eq!((5, 0), possible_moves[4].to);
        assert_eq!((1, 6), possible_moves[5].to);
        assert_eq!((2, 7), possible_moves[6].to);

        // Default movements for Knight at c8
        possible_moves = board.generate_bishop_moves(default_bishop_positions[2]);
        assert_eq!(7, possible_moves.len());
        assert_eq!((6, 1), possible_moves[0].to);
        assert_eq!((5, 0), possible_moves[1].to);
        assert_eq!((5, 4), possible_moves[3].to);
        assert_eq!((4, 5), possible_moves[4].to);
        assert_eq!((3, 6), possible_moves[5].to);
        assert_eq!((2, 7), possible_moves[6].to);

        // Default movements for Knight at c8
        possible_moves = board.generate_bishop_moves(default_bishop_positions[3]);
        assert_eq!(7, possible_moves.len());
        assert_eq!((6, 4), possible_moves[0].to);
        assert_eq!((5, 3), possible_moves[1].to);
        assert_eq!((3, 1), possible_moves[3].to);
        assert_eq!((2, 0), possible_moves[4].to);
        assert_eq!((6, 6), possible_moves[5].to);
        assert_eq!((5, 7), possible_moves[6].to);
    }

    #[test]
    fn test_generate_bishop_moves_x() {
        let board = ChessBoard::from_str("8/3p4/8/5B2/3b4/8/2P5/8 w - - 0 0").unwrap();

        // Available movements for Bishop at d4
        let mut possible_moves = board.generate_bishop_moves((4, 3));
        assert_eq!(13, possible_moves.len());
        assert_eq!((3, 2), possible_moves[0].to);
        assert_eq!((2, 1), possible_moves[1].to);
        assert_eq!((3, 4), possible_moves[3].to);
        assert_eq!((2, 5), possible_moves[4].to);
        assert_eq!((1, 6), possible_moves[5].to);
        assert_eq!((0, 7), possible_moves[6].to);
        assert_eq!((5, 2), possible_moves[7].to);
        assert_eq!((6, 1), possible_moves[8].to);
        assert_eq!((7, 0), possible_moves[9].to);
        assert_eq!((5, 4), possible_moves[10].to);
        assert_eq!((6, 5), possible_moves[11].to);
        assert_eq!((7, 6), possible_moves[12].to);

        // Available movements for Bishop at f5
        possible_moves = board.generate_bishop_moves((3, 5));
        assert_eq!(7, possible_moves.len());
        assert_eq!((2, 4), possible_moves[0].to);
        assert_eq!((2, 6), possible_moves[1].to);
        assert_eq!((4, 4), possible_moves[3].to);
        assert_eq!((5, 3), possible_moves[4].to);
        assert_eq!((4, 6), possible_moves[5].to);
        assert_eq!((5, 7), possible_moves[6].to);
    }
}
