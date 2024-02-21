#[cfg(test)]
mod tests {
    use ajedrez::Color;
    use std::str::FromStr;

    #[test]
    fn test_color_from_str() {
        assert_eq!(Color::from_str("w").unwrap(), Color::White);
        assert_eq!(Color::from_str("W").unwrap(), Color::White);
        assert_eq!(Color::from_str("White").unwrap(), Color::White);
        assert_eq!(Color::from_str("white").unwrap(), Color::White);
        assert_eq!(Color::from_str("wow").unwrap(), Color::White);
        assert_eq!(Color::from_str("whi").unwrap(), Color::White);
        assert_eq!(Color::from_str("b").unwrap(), Color::Black);
        assert_eq!(Color::from_str("B").unwrap(), Color::Black);
        assert_eq!(Color::from_str("Black").unwrap(), Color::Black);
        assert_eq!(Color::from_str("black").unwrap(), Color::Black);
        assert_eq!(Color::from_str("bl").unwrap(), Color::Black);
        assert_eq!(Color::from_str("bluetooth").unwrap(), Color::Black);

        // Invalid input string
        assert!(Color::from_str("Nope").is_err());
        assert!(Color::from_str("").is_err());
    }
}
