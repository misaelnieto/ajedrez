#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use ajedrez::{ChessBoard, Color};

    #[test]
    fn test_constructor() {
        let mut board = ChessBoard::new();
        assert_eq!("8/8/8/8/8/8/8/8 w - - 0 0", board.as_fen());
    }

    #[test]
    fn test_load_from_fen_code() {
        let board = ChessBoard::from_str("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
        // 8th rank
        assert_eq!('r', board.get_piece(8, &'a').unwrap().as_fen());
        assert_eq!('n', board.get_piece(8, &'b').unwrap().as_fen());
        assert_eq!('b', board.get_piece(8, &'c').unwrap().as_fen());
        assert_eq!('q', board.get_piece(8, &'d').unwrap().as_fen());
        assert_eq!('k', board.get_piece(8, &'e').unwrap().as_fen());
        assert_eq!('b', board.get_piece(8, &'f').unwrap().as_fen());
        assert_eq!('n', board.get_piece(8, &'g').unwrap().as_fen());
        assert_eq!('r', board.get_piece(8, &'h').unwrap().as_fen());
        // 7th rank
        assert_eq!('p', board.get_piece(7, &'a').unwrap().as_fen());
        assert_eq!('p', board.get_piece(7, &'b').unwrap().as_fen());
        assert_eq!('p', board.get_piece(7, &'c').unwrap().as_fen());
        assert_eq!('p', board.get_piece(7, &'d').unwrap().as_fen());
        assert_eq!('p', board.get_piece(7, &'e').unwrap().as_fen());
        assert_eq!('p', board.get_piece(7, &'f').unwrap().as_fen());
        assert_eq!('p', board.get_piece(7, &'g').unwrap().as_fen());
        assert_eq!('p', board.get_piece(7, &'h').unwrap().as_fen());
        // 6th rank
        assert_eq!(None, board.get_piece(6, &'a'));
        assert_eq!(None, board.get_piece(6, &'b'));
        assert_eq!(None, board.get_piece(6, &'c'));
        assert_eq!(None, board.get_piece(6, &'d'));
        assert_eq!(None, board.get_piece(6, &'e'));
        assert_eq!(None, board.get_piece(6, &'f'));
        assert_eq!(None, board.get_piece(6, &'g'));
        assert_eq!(None, board.get_piece(6, &'h'));
        // 5th rank
        assert_eq!(None, board.get_piece(5, &'a'));
        assert_eq!(None, board.get_piece(5, &'b'));
        assert_eq!(None, board.get_piece(5, &'c'));
        assert_eq!(None, board.get_piece(5, &'d'));
        assert_eq!(None, board.get_piece(5, &'e'));
        assert_eq!(None, board.get_piece(5, &'f'));
        assert_eq!(None, board.get_piece(5, &'g'));
        assert_eq!(None, board.get_piece(5, &'h'));
        // 4th rank
        assert_eq!(None, board.get_piece(4, &'a'));
        assert_eq!(None, board.get_piece(4, &'b'));
        assert_eq!(None, board.get_piece(4, &'c'));
        assert_eq!(None, board.get_piece(4, &'d'));
        assert_eq!('P', board.get_piece(4, &'e').unwrap().as_fen());
        assert_eq!(None, board.get_piece(4, &'f'));
        assert_eq!(None, board.get_piece(4, &'g'));
        assert_eq!(None, board.get_piece(4, &'h'));
        // 3th rank
        assert_eq!(None, board.get_piece(3, &'a'));
        assert_eq!(None, board.get_piece(3, &'b'));
        assert_eq!(None, board.get_piece(3, &'c'));
        assert_eq!(None, board.get_piece(3, &'d'));
        assert_eq!(None, board.get_piece(3, &'e'));
        assert_eq!(None, board.get_piece(3, &'f'));
        assert_eq!(None, board.get_piece(3, &'g'));
        assert_eq!(None, board.get_piece(3, &'h'));
        // 2th rank
        assert_eq!('P', board.get_piece(2, &'a').unwrap().as_fen());
        assert_eq!('P', board.get_piece(2, &'b').unwrap().as_fen());
        assert_eq!('P', board.get_piece(2, &'c').unwrap().as_fen());
        assert_eq!('P', board.get_piece(2, &'d').unwrap().as_fen());
        assert_eq!(None, board.get_piece(2, &'e'));
        assert_eq!('P', board.get_piece(2, &'f').unwrap().as_fen());
        assert_eq!('P', board.get_piece(2, &'g').unwrap().as_fen());
        assert_eq!('P', board.get_piece(2, &'h').unwrap().as_fen());
        // 1st rank
        assert_eq!('R', board.get_piece(1, &'a').unwrap().as_fen());
        assert_eq!('N', board.get_piece(1, &'b').unwrap().as_fen());
        assert_eq!('B', board.get_piece(1, &'c').unwrap().as_fen());
        assert_eq!('Q', board.get_piece(1, &'d').unwrap().as_fen());
        assert_eq!('K', board.get_piece(1, &'e').unwrap().as_fen());
        assert_eq!('B', board.get_piece(1, &'f').unwrap().as_fen());
        assert_eq!('N', board.get_piece(1, &'g').unwrap().as_fen());
        assert_eq!('R', board.get_piece(1, &'h').unwrap().as_fen());

        // Current turn/ active color
        assert_eq!(Color::Black, board.active_color());

        // TODO: Castling
        // En passant
        assert_eq!("-", board.passant_square());
        // Half moves
        assert_eq!(0, board.half_moves());
        // Full moves
        assert_eq!(1, board.full_moves());
    }
}