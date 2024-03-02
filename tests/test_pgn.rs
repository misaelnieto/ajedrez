#[cfg(test)]
mod tests {
    use ajedrez::PGNGame;
    use std::{fs, io};

    #[test]
    fn test_game1() -> io::Result<()> {
        let input = fs::read_to_string("tests/pgn_games/game1.pgn")
            .expect("Can't open anastasian-lewis.pgn");
        let game = PGNGame::new(&input);
        game.unwrap().play();
        Ok(())
    }

    #[test]
    fn test_morphy_karl_isouard_1858() -> io::Result<()> {
        let input = fs::read_to_string("tests/pgn_games/morphy_karl_isouard_1858.pgn")
            .expect("Can't open PGN file");
        let game = PGNGame::new(&input);
        game.unwrap().play();
        Ok(())
    }

    #[test]
    fn test_byrne_fischer_1956() -> io::Result<()> {
        let input = fs::read_to_string("tests/pgn_games/byrne_fischer_1956.pgn")
            .expect("Can't open PGN file");
        let game = PGNGame::new(&input);
        game.unwrap().play();
        Ok(())
    }

    #[test]
    fn test_andersen_kieseritzky_1851() -> io::Result<()> {
        let input = fs::read_to_string("tests/pgn_games/andersen_kieseritzky_1851.pgn")
            .expect("Can't open PGN file");
        let game = PGNGame::new(&input);
        game.unwrap().play();
        Ok(())
    }
}
