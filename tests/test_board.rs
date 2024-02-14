#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ajedrez::{ChessBoard, Color, Piece, PieceType, Square};


    #[test]
    fn test_piece_as_board_coordinate() {
        let mut sq = Square::default();
        assert_eq!("-", format!("{sq}"));
        sq = Square { piece: Option::from(Piece::new(Color::White, PieceType::King)), rank: 8, file: 'a'};
        assert_eq!("8a", format!("{sq}"));
        sq = Square { piece: Option::from(Piece::new(Color::White, PieceType::King)), rank: 2, file: 'c'};
        assert_eq!("2c", format!("{sq}"));
    }

}