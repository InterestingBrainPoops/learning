use std::str::FromStr;

// Chess board
#[derive(Debug)]
pub struct Board {
    pub pieces: Vec<Piece>,
    pub turn: Color,
}
// Piece implementation/ representation
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece: PieceType,
    color: Color,
    position: u8,
}
// Piece type (does this need more explanation)
#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Empty,
    King(bool), // moved or not (if a king or the rook its trying to castle with have moved, then it cannot castle.)
    Queen,
    Bishop,
    Knight,
    Rook(bool), // moved or not
    Pawn(Option<u8>), // can en-passant
}
// color of a piece, didn't wanna use a boolean.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}
pub struct Move {
    piece: Piece,           // the piece itself
    move_by: u8,            // how much its moving by
    capture: Option<Piece>, // if capture put the captured piece here
    castle: Option<Piece>,  // if castling, put the rook here
    promote: Option<Piece>, // if promoted, to what piece.
}

impl Board {
    // sets the board to the fen string
    pub fn from_fen(&mut self, fen: &String) {
        // split the FEN into is constituent parts.
        let input: Vec<&str> = fen.split(" ").collect();
        let fen = String::from(input[0]);
		println!("{}", fen);
        // king can always castle, unless it has moved from its inital position.
        // iff king and rook can castle, then and only then can they castle.
        let mut row = 0;
        let mut col = 0;
        for x in fen.split("") {
            let parsed = u8::from_str(x);
            match parsed {
                Ok(moveby) => col += moveby,
                Err(_) => {
                    let color: Color;
                    let mut out = Piece {
                        piece: PieceType::Empty,
                        color: Color::Black,
                        position: 3,
                    };
                    match x.to_lowercase().as_str() {
                        "r" => {
                            let can_castle;
                            // color logic
                            // Check the case of x.
                            if x == "R" {
                                color = Color::White;
                            } else {
                                color = Color::Black;
                            }

                            match row * 8 + col {
                                // can castle logic
                                0 => {
                                    if input[2].contains("q") {
                                        can_castle = true;
                                    } else {
                                        can_castle = false;
                                    }
                                }
                                56 => {
                                    if input[2].contains("Q") {
                                        can_castle = true;
                                    } else {
                                        can_castle = false;
                                    }
                                }
                                63 => {
                                    if input[2].contains("K") {
                                        can_castle = true;
                                    } else {
                                        can_castle = false;
                                    }
                                }
                                7 => {
                                    if input[2].contains("k") {
                                        can_castle = true;
                                    } else {
                                        can_castle = false;
                                    }
                                }
                                _ => {
                                    can_castle = false;
                                }
                            }
                            out = Piece {
                                piece: PieceType::Rook(can_castle),
                                color: color.clone(),
                                position: row * 8 + col,
                            };col += 1;
                        } // rook logic
                        "b" => {
                            if x == "B" {
                                color = Color::White;
                            } else {
                                color = Color::Black;
                            }
                            out = Piece {
                                piece: PieceType::Bishop,
                                color,
                                position: row * 8 + col,
                            };col += 1;
                        } // bishop logic
                        "n" => {
                            if x == "N" {
                                color = Color::White;
                            } else {
                                color = Color::Black;
                            }
                            out = Piece {
                                piece: PieceType::Knight,
                                color,
                                position: row * 8 + col,
                            };col += 1;
                        } // knight logic
                        "q" => {
                            if x == "Q" {
                                color = Color::White;
                            } else {
                                color = Color::Black;
                            }
                            out = Piece {
                                piece: PieceType::Queen,
                                color,
                                position: row * 8 + col,
                            };col += 1;
                        } // queen logic
						"p" => {
                            if x == "P" {
                                color = Color::White;
                            } else {
                                color = Color::Black;
                            }
							let mut canenpassant = None;
							let mut rank = 0;
							if input[3] != "-" {
								for thing in input[3].split(""){
									match thing.parse() {
										Ok(num) => {canenpassant = Some(num); continue;},
										Err(_) => {},
									};
									match thing {
										"a" => {rank = 0;}
										"b" => {rank = 1;}
										"c" => {rank = 2;}
										"d" => {rank = 3;}
										"e" => {rank = 4;}
										"f" => {rank = 5;}
										"g" => {rank = 6;}
										"h" => {rank = 7;}
										_ => {panic!("Invalid character. {}", thing)}
									}
								}
							}
                            out = Piece {
                                piece: PieceType::Pawn(canenpassant),
                                color,
                                position: row * 8 + col,
                            };col += 1;
                        } // queen logic
                        "k" => {
                            if x == "K" {
                                color = Color::White;
                            } else {
                                color = Color::Black;
                            }
                            out = Piece {
                                piece: PieceType::King(true),
                                color,
                                position: row * 8 + col,
                            };col += 1;
                        } // king logic
                        "/" => {
                            col = 0;
                            row += 1;
                        } // end of line
						"" =>{

						}// nothing happens at end of string.
                        _ => {
                            panic!("Invalid char in FEN string : {}", x);
                        }
                    }
					
                    if !matches!(out.piece, PieceType::Empty) {
                        self.pieces.push(out.clone());
                    }
                }
            }
        }
    }
    // gets the valid moves for a given position
    // IDK how to do this yet
    fn valid_moves(&self) -> Vec<Move> {
        let out: Vec<Move> = vec![];
        out
    }
    // makes a given move
    // removes taken pieces.
    fn make_move(&self, the_move: Move) {}
    // unmakes a given move
    // replaces pieces that were captured if necessary.
    fn unmake_move(&self, the_move: Move) {}
    // checks if a state is an end state
    // if it is then return the color of winner as well.
    fn end(&self) -> (bool, Option<Color>) {
        (false, None)
    }
    // static evaluation of a given board state
    fn evaluation(&self) -> i32 {
        0
    }
}
