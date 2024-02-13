use std::collections::HashMap;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::{ChessBoard, ChessMove, ChessMoveError, Color, File2Index, Move, PieceType, Rank2Index, rank_to_index};
use crate::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

#[derive(Parser)]
#[grammar = "pgn.pest"]
pub struct PGNParser;


const INITIAL_FEN_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 0";

pub struct PieceMove<'a> {
    piece: PieceType,
    color: Color,
    from_file: i8,
    from_rank: i8,
    to_file: i8,
    to_rank: i8,

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
            from_file: -1,
            from_rank: -1,
            to_file: -1,
            to_rank: -1,
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
                        _ => Pawn
                    }
                }
                Rule::from_file => {
                    mp.from_file = part.as_str().file_to_zero_base_index().unwrap() as i8;
                }
                Rule::from_rank => {
                    mp.from_rank = part.as_str().rank_to_zero_base_index().unwrap() as i8;
                }
                Rule::to_file => {
                    mp.to_file = part.as_str().file_to_zero_base_index().unwrap() as i8;
                }
                Rule::to_rank => {
                    mp.to_rank = rank_to_index(part.as_str().parse::<usize>().unwrap()) as i8;
                }
                _ => todo!("Unexpected rule!")
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
            .next().unwrap();
        let mut g = PGNGame {
            board: ChessBoard::from_str(INITIAL_FEN_BOARD).expect("Error parsing initial FEN board"),
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
            println!("{}", self.process_move_pair(ix, &full_move).expect("Full move should be valid"));
            println!("{}", self.board.as_str());
        }

        println!("---------------------------------------------");
        println!("| Game Result: {}", self.game_result);
        println!("---------------------------------------------");
    }

    pub fn process_move_pair(&mut self, move_ix: usize, full_move: &Pair<Rule>) -> Result<String, ChessMoveError> {
        let complete_moves: Vec<Pair<Rule>> = full_move.clone()
            .into_inner()
            .filter(|p| p.as_rule() == Rule::complete_move)
            .into_iter().collect();
        // White
        let mut log_str = self.process_complete_move(move_ix, Color::White, &complete_moves[0]).unwrap();
        log_str = format!("{}: White {}", complete_moves[0].as_str(), log_str);

        // Black
        if complete_moves.len() > 1 {
            log_str = format!("{} | {}: Black {}",
                              log_str,
                              complete_moves[1].as_str(),
                              self.process_complete_move(move_ix, Color::Black, &complete_moves[1]).unwrap());
        }
        Ok(log_str)
    }
    pub fn process_complete_move(&mut self, move_ix: usize, player_color: Color, full_move: &Pair<Rule>) -> Result<String, ChessMoveError> {
        let move_or_castle = full_move.clone().into_inner().next().expect("Unexpected empty rule pair");
        match move_or_castle.as_rule() {
            Rule::move_piece => {
                let mut movement = PieceMove::from_rule(move_or_castle.into_inner().next().expect("Unexpected empty rule pair"));
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
        let available_pieces = self.board.find_pieces(movement.piece, movement.color);
        if available_pieces.is_empty() {
            return Err(ChessMoveError::StartPieceMissing);
        }

        // Find the starting square. We should have a good candidate for the starting square after this loop
        for p in available_pieces {
            for mv in self.board.generate_intrinsic_moves(p.to_zero_based_index()) {
                if (mv.to.0, mv.to.1) == (movement.to_rank as usize, movement.to_file as usize) {
                    (movement.from_rank, movement.from_file) = (mv.from.0 as i8, mv.from.1 as i8);
                    break;
                }
            }
        }

        // We still don't have a starting square yet? We fail
        if movement.from_file < 0 || movement.from_rank <0 {
            return Err(ChessMoveError::StartPieceMissing)
        }

        // Do the move!
        self.board.move_piece(Move {
            from: (movement.from_rank as usize, movement.from_file as usize),
            to: (movement.to_rank as usize, movement.to_file as usize),
            castling: false,
        })
    }
}

