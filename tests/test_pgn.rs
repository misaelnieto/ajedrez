#[cfg(test)]
mod tests {
    use std::{fs, io};
    use ajedrez::PGNGame;

    #[test]
    fn test_game1()  -> io::Result<()>  {
        let input = fs::read_to_string("tests/pgn_games/game1.pgn").expect("Can't open anastasian-lewis.pgn");
        let game = PGNGame::new(&input);
        game.unwrap().play();
        Ok(())
    }

    #[test]
    fn test_anastasian_lewis()  -> io::Result<()>  {
        let input = fs::read_to_string("tests/pgn_games/anastasian-lewis.pgn").expect("Can't open anastasian-lewis.pgn");
        let game = PGNGame::new(&input);
        game.unwrap().play();
        Ok(())
    }
}