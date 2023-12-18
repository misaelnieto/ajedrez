#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ajedrez::{Move, ParseError};

    #[test]
    fn test_board_fen_code() {
        // Invalid strings
        assert_eq!(Err(ParseError::StringTooShort), Move::from_str("x"));
        assert_eq!(Err(ParseError::StringTooShort), Move::from_str("123"));
        assert_eq!(Err(ParseError::InvalidPositionFile), Move::from_str("i1i1"));
        assert_eq!(Err(ParseError::InvalidPositionFile), Move::from_str("01i1"));
        assert_eq!(Err(ParseError::InvalidPositionRank), Move::from_str("a0a0"));
        assert_eq!(Err(ParseError::InvalidPositionRank), Move::from_str("aza0"));
        assert_eq!(Err(ParseError::InvalidPositionRank), Move::from_str("a1h9"));

        let mut mov = Move::from_str("a8b8").expect("parsing move should not have failed");
        assert_eq!((0,0), mov.from);
        assert_eq!((0,1), mov.to);
        mov = Move::from_str("a1a8").expect("parsing move should not have failed");
        assert_eq!((7,0), mov.from);
        assert_eq!((0,0), mov.to);
    }
}
