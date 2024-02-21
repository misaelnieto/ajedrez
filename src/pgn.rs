use std::collections::HashMap;

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::fen::INITIAL_FEN_BOARD;
use crate::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::{
    rank_to_index, ChessBoard, ChessMove, ChessMoveError, Color, FENStringParsing, File2Index,
    Move, PieceType,
};

#[derive(Parser)]
#[grammar = "pgn.pest"]
struct PGNParser;

pub struct PieceMove<'a> {
    piece: PieceType,
    color: Color,
    from_row: i8,
    from_col: i8,
    to_row: i8,
    to_col: i8,
    row_disambiguator: i8,
    col_disambiguator: i8,

    // Fields are useful for debugging purposes
    #[allow(dead_code)]
    rule: Rule,
    #[allow(dead_code)]
    as_str: &'a str,
    #[allow(dead_code)]
    move_ix: usize,
}

impl PieceMove<'_> {
    pub fn from_rule(parsed_move: Pair<Rule>) -> PieceMove {
        let mut mp = PieceMove {
            piece: PieceType::Pawn,
            color: Color::White,
            from_row: -1,
            from_col: -1,
            to_row: -1,
            to_col: -1,
            row_disambiguator: -1,
            col_disambiguator: -1,
            rule: parsed_move.as_rule(),
            as_str: parsed_move.as_str(),
            move_ix: 0,
        };

        for part in parsed_move.into_inner().into_iter() {
            match part.as_rule() {
                Rule::piece => {
                    mp.piece = match part.as_str() {
                        "K" => King,
                        "Q" => Queen,
                        "R" => Rook,
                        "B" => Bishop,
                        "N" => Knight,
                        _ => Pawn,
                    }
                }
                Rule::disambiguator => {
                    let d = part.as_str();
                    let value = d.file_to_zero_base_index();
                    if value.is_ok() {
                        mp.col_disambiguator = d.file_to_zero_base_index().unwrap() as i8;
                    } else {
                        mp.row_disambiguator = d.file_to_zero_base_index().unwrap() as i8;
                    }
                }
                Rule::to_file => {
                    mp.to_col = part.as_str().file_to_zero_base_index().unwrap() as i8;
                }
                Rule::to_rank => {
                    mp.to_row = rank_to_index(part.as_str().parse::<usize>().unwrap()) as i8;
                }

                _ => todo!("Unexpected rule!"),
            }
        }
        mp
    }
}

pub struct PGNGame<'a> {
    board: ChessBoard,
    metadata: HashMap<String, String>,
    game_result: String,
    moves: Vec<Pair<'a, Rule>>,
}

impl<'a> PGNGame<'a> {
    pub fn new(pgn_str: &'a str) -> Option<PGNGame<'a>> {
        let parsed_pgn = PGNParser::parse(Rule::game, &pgn_str)
            .expect("Invalid PGN file") // unwrap the parse result
            .next()
            .unwrap();
        let mut g = PGNGame {
            board: INITIAL_FEN_BOARD
                .parse_fen()
                .expect("Error parsing initial FEN board"),
            metadata: HashMap::new(),
            game_result: String::new(),
            moves: Vec::new(),
        };

        for child_node in parsed_pgn.into_inner() {
            match child_node.as_rule() {
                Rule::metadata_block => {
                    let mut inner_pairs = child_node.into_inner();
                    let key = inner_pairs.next().unwrap().as_str().to_string();
                    let value = inner_pairs.next().unwrap().as_str().to_string();
                    g.metadata.insert(key, value);
                }
                Rule::game_result => {
                    let inner_pairs = child_node.into_inner();
                    g.game_result = String::from(inner_pairs.as_str());
                }
                Rule::move_list => {
                    g.moves.extend(child_node.into_inner());
                }
                _ => {
                    println!("Unknown rule {:?}", child_node);
                }
            }
        }

        Some(g)
    }

    pub fn play(mut self) {
        println!("---------------------------------------------");
        println!("| Game metadata                              ");
        for (key, value) in &self.metadata {
            println!("| {: <20} | {: <10}", key, value);
        }

        println!("---------------------------------------------");
        println!("| Game starts!                               ");
        for ix in 0..self.moves.len() {
            print!("{:3} Move -> ", ix + 1);
            // Access the full_move by index. Clone it to avoid borrowing issues.
            let full_move = self.moves[ix].clone();
            println!(
                "{}",
                self.process_move_pair(ix, &full_move)
                    .expect("Full move should be valid")
            );
            println!("{}", self.board.as_str());
        }

        println!("---------------------------------------------");
        println!("| Game Result: {}", self.game_result);
        println!("---------------------------------------------");
    }

    pub fn process_move_pair(
        &mut self,
        move_ix: usize,
        full_move: &Pair<Rule>,
    ) -> Result<String, ChessMoveError> {
        let complete_moves: Vec<Pair<Rule>> = full_move
            .clone()
            .into_inner()
            .filter(|p| p.as_rule() == Rule::complete_move)
            .into_iter()
            .collect();
        // White
        let mut log_str = self
            .process_complete_move(move_ix, Color::White, &complete_moves[0])
            .unwrap();
        log_str = format!("{}: White {}", complete_moves[0].as_str(), log_str);

        // Black
        if complete_moves.len() > 1 {
            log_str = format!(
                "{} | {}: Black {}",
                log_str,
                complete_moves[1].as_str(),
                self.process_complete_move(move_ix, Color::Black, &complete_moves[1])
                    .unwrap()
            );
        }
        Ok(log_str)
    }

    pub fn process_complete_move(
        &mut self,
        move_ix: usize,
        player_color: Color,
        full_move: &Pair<Rule>,
    ) -> Result<String, ChessMoveError> {
        let move_or_castle = full_move
            .clone()
            .into_inner()
            .next()
            .expect("Unexpected empty rule pair");
        match move_or_castle.as_rule() {
            Rule::move_piece => {
                let mut movement = PieceMove::from_rule(
                    move_or_castle
                        .into_inner()
                        .next()
                        .expect("Unexpected empty rule pair"),
                );
                movement.color = player_color;
                movement.move_ix = move_ix;
                return self.infer_move(&mut movement);
            }
            Rule::castle_kingside => {
                return self.board.castle(player_color, ChessMove::CastleKingside)
            }
            Rule::castle_queenside => {
                return self.board.castle(player_color, ChessMove::CastleQueenside)
            }
            _ => {}
        }
        Ok(String::new())
    }

    fn infer_move(&mut self, movement: &mut PieceMove) -> Result<String, ChessMoveError> {
        let mut available_pieces = self.board.find_pieces(movement.piece, movement.color);

        // If we have a disambiguator, discard pieces that are not from that column
        if movement.row_disambiguator > 0 {
            available_pieces.retain(|sq| sq.row == movement.row_disambiguator as usize);
        }
        if movement.col_disambiguator > 0 {
            available_pieces.retain(|sq| sq.col == movement.col_disambiguator as usize);
        }

        // Fail early
        if available_pieces.is_empty() {
            return Err(ChessMoveError::StartPieceMissing);
        }

        // If we are down to one piece, we are done!
        if available_pieces.len() == 1 {
            movement.from_col = available_pieces[0].col as i8;
            movement.from_row = available_pieces[0].row as i8;
        } else {
            // Ok we have more work left to be done.
            // Gather all possible moves from the all the available pieces
            let mut possible_moves: Vec<Move> = Vec::new();
            for p in available_pieces {
                let mut i_moves = self.board.generate_intrinsic_moves((p.row, p.col));
                // Discard moves that don't go to our target square
                i_moves.retain(|mv| {
                    (mv.to.0, mv.to.1) == (movement.to_row as usize, movement.to_col as usize)
                });
                possible_moves.extend(i_moves.iter());
            }

            if possible_moves.len() == 1 {
                movement.from_row = possible_moves[0].from.0 as i8;
                movement.from_col = possible_moves[0].from.1 as i8;
            } else {
                // We still have multiple movements to chose from.
                return Err(ChessMoveError::TooManyPossibleMoves);
            }
        }

        // Do the move!
        self.board.move_piece(Move {
            from: (movement.from_row as usize, movement.from_col as usize),
            to: (movement.to_row as usize, movement.to_col as usize),
            castling: false,
        })
    }
}
