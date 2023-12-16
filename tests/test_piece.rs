#[cfg(test)]
mod test {
    use ajedrez::{Color, Piece, PieceType};

    #[test]
    fn test_piece_as_fen() {
        let mut p  = Piece::new(Color::White, PieceType::King);
        assert_eq!('K', p.as_fen());
        p  = Piece::new(Color::Black, PieceType::King);
        assert_eq!('k', p.as_fen());
    }
}