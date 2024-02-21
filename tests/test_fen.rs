#[cfg(test)]
mod tests {
    use ajedrez::{BoardAsFEN, ChessBoard, Color, FENStringParsing, PieceType, INITIAL_FEN_BOARD};

    #[test]
    fn test_board_parse_fen() {
        let board = match INITIAL_FEN_BOARD.parse_fen() {
            Ok(b) => b,
            Err(e) => {
                panic!("Parsing failed with the following error: {e:?}")
            }
        };

        // 8th rank
        assert_eq!('r', board.get_piece_a("a8").unwrap().as_fen());
        assert_eq!('n', board.get_piece_a("b8").unwrap().as_fen());
        assert_eq!('b', board.get_piece_a("c8").unwrap().as_fen());
        assert_eq!('q', board.get_piece_a("d8").unwrap().as_fen());
        assert_eq!('k', board.get_piece_a("e8").unwrap().as_fen());
        assert_eq!('b', board.get_piece_a("f8").unwrap().as_fen());
        assert_eq!('n', board.get_piece_a("g8").unwrap().as_fen());
        assert_eq!('r', board.get_piece_a("h8").unwrap().as_fen());
        // 7th rank
        assert_eq!('p', board.get_piece_a("a7").unwrap().as_fen());
        assert_eq!('p', board.get_piece_a("b7").unwrap().as_fen());
        assert_eq!('p', board.get_piece_a("c7").unwrap().as_fen());
        assert_eq!('p', board.get_piece_a("d7").unwrap().as_fen());
        assert_eq!('p', board.get_piece_a("e7").unwrap().as_fen());
        assert_eq!('p', board.get_piece_a("f7").unwrap().as_fen());
        assert_eq!('p', board.get_piece_a("g7").unwrap().as_fen());
        assert_eq!('p', board.get_piece_a("h7").unwrap().as_fen());
        // 6th rank
        assert_eq!(None, board.get_piece_a("a6"));
        assert_eq!(None, board.get_piece_a("b6"));
        assert_eq!(None, board.get_piece_a("c6"));
        assert_eq!(None, board.get_piece_a("d6"));
        assert_eq!(None, board.get_piece_a("e6"));
        assert_eq!(None, board.get_piece_a("f6"));
        assert_eq!(None, board.get_piece_a("g6"));
        assert_eq!(None, board.get_piece_a("h6"));
        // 5th rank
        assert_eq!(None, board.get_piece_a("a5"));
        assert_eq!(None, board.get_piece_a("b5"));
        assert_eq!(None, board.get_piece_a("c5"));
        assert_eq!(None, board.get_piece_a("d5"));
        assert_eq!(None, board.get_piece_a("e5"));
        assert_eq!(None, board.get_piece_a("f5"));
        assert_eq!(None, board.get_piece_a("g5"));
        assert_eq!(None, board.get_piece_a("h5"));
        // 4th rank
        assert_eq!(None, board.get_piece_a("a4"));
        assert_eq!(None, board.get_piece_a("b4"));
        assert_eq!(None, board.get_piece_a("c4"));
        assert_eq!(None, board.get_piece_a("d4"));
        assert_eq!(None, board.get_piece_a("e4"));
        assert_eq!(None, board.get_piece_a("f4"));
        assert_eq!(None, board.get_piece_a("g4"));
        assert_eq!(None, board.get_piece_a("h4"));
        // 3th rank
        assert_eq!(None, board.get_piece_a("a3"));
        assert_eq!(None, board.get_piece_a("b3"));
        assert_eq!(None, board.get_piece_a("c3"));
        assert_eq!(None, board.get_piece_a("d3"));
        assert_eq!(None, board.get_piece_a("e3"));
        assert_eq!(None, board.get_piece_a("f3"));
        assert_eq!(None, board.get_piece_a("g3"));
        assert_eq!(None, board.get_piece_a("h3"));
        // 2th rank
        assert_eq!('P', board.get_piece_a("a2").unwrap().as_fen());
        assert_eq!('P', board.get_piece_a("b2").unwrap().as_fen());
        assert_eq!('P', board.get_piece_a("c2").unwrap().as_fen());
        assert_eq!('P', board.get_piece_a("d2").unwrap().as_fen());
        assert_eq!('P', board.get_piece_a("e2").unwrap().as_fen());
        assert_eq!('P', board.get_piece_a("f2").unwrap().as_fen());
        assert_eq!('P', board.get_piece_a("g2").unwrap().as_fen());
        assert_eq!('P', board.get_piece_a("h2").unwrap().as_fen());
        // 1st rank
        assert_eq!('R', board.get_piece_a("a1").unwrap().as_fen());
        assert_eq!('N', board.get_piece_a("b1").unwrap().as_fen());
        assert_eq!('B', board.get_piece_a("c1").unwrap().as_fen());
        assert_eq!('Q', board.get_piece_a("d1").unwrap().as_fen());
        assert_eq!('K', board.get_piece_a("e1").unwrap().as_fen());
        assert_eq!('B', board.get_piece_a("f1").unwrap().as_fen());
        assert_eq!('N', board.get_piece_a("g1").unwrap().as_fen());
        assert_eq!('R', board.get_piece_a("h1").unwrap().as_fen());

        // Current turn/ active color
        assert_eq!(Color::White, board.active_color);

        // TODO: Castling
        // En passant
        assert_eq!(None, board.passant_square);
        // Half moves
        assert_eq!(0, board.half_moves);
        // Full moves
        assert_eq!(0, board.full_moves);
    }

    #[test]
    fn test_board_as_fen_0() {
        let mut board = ChessBoard::new();
        assert_eq!("8/8/8/8/8/8/8/8 w - - 0 0", board.as_fen());

        // Set up the pieces on the chessboard
        board.set_piece(8, &'a', Color::White, PieceType::Rook);
        board.set_piece(8, &'h', Color::White, PieceType::Rook);
        board.set_piece(1, &'a', Color::Black, PieceType::Rook);
        board.set_piece(1, &'h', Color::Black, PieceType::Rook);

        assert_eq!("R6R/8/8/8/8/8/8/r6r w - - 0 0", board.as_fen());
    }

    #[test]
    fn test_board_as_fen_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(fen, fen.parse_fen().unwrap().as_fen());

        let fen = "8/8/2rbk3/3P4/8/8/8/8 w - - 0 0";
        assert_eq!(fen, fen.parse_fen().unwrap().as_fen());
    }
}
