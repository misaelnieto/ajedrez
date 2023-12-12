#[cfg(test)]
mod test {
    use ajedrez::{Color, Piece, PieceType};

    #[test]
    fn test_piece_as_fen() {
        let mut p  = Piece::new(Color::White, PieceType::King, 0, 0);
        assert_eq!('K', p.as_fen());
        p  = Piece::new(Color::Black, PieceType::King, 0, 0);
        assert_eq!('k', p.as_fen());
    }

    #[test]
    fn test_piece_as_board_coordinate() {
        let mut p  = Piece::new(Color::White, PieceType::King, 0, 0);
        assert_eq!("a0", p.board_coordinate());
        p  = Piece::new(Color::Black, PieceType::King, 2, 2);
        assert_eq!("c2", p.board_coordinate());
    }
}