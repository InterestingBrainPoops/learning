use std::str::FromStr;

// Chess board
pub struct Board{
	pub pieces : Vec<Piece>,
	pub turn : Color
}
// Piece implementation/ representation
pub struct Piece{
	piece : PieceType,
	color : Color,
	position : u8
}
// Piece type (does this need more explanation)
pub enum PieceType{
	King(bool), // moved or not (if a king or the rook its trying to castle with have moved, then it cannot castle.)
	Queen,
	Bishop,
	Knight,
	Rook(bool), // moved or not
	Pawn(bool) // can en-passant
}
// color of a piece, didn't wanna use a boolean.
pub enum Color{
	Black,
	White
}
pub struct Move{
	piece : Piece, // the piece itself
	move_by : u8, // how much its moving by
	capture : Option<Piece>, // if capture put the captured piece here
	castle : Option<Piece>, // if castling, put the rook here
	promote : Option<Piece> // if promoted, to what piece.
}

impl Board{
	// sets the board to the fen string
	pub fn from_fen(&self, fen:&String){
		// split the FEN into is constituent parts.
		let input: Vec<&str> = fen.split(" ").collect();
		let fen = String::from(input[0]);
		let cpos = 0;
		// println!("{:?}", fen);
		// [board string, to move, castle ability, en passant square, half move clock, full move clock]
		// iterate through the board string
			// match char type:
				// int
					// increase the current position by that amount.
				// char
					// determine if black or white
					// match char across [r,n,b,q,k,/]
						// if its a king or rook check the `castle ability` option to see if the king can or cannot castle. (and set the bool.)
							// if a certain side cannot castle, then set the king castle value to false.
							// otherwise ONLY modify the rook castle ability.
						// _ => {}
					// increase the current position by 1.
		let mut row = 0;
		let mut col = 0;
		for x in fen.split(""){
			let parsed = u8::from_str(x);
			match parsed{
				Ok(moveby) => {col += moveby},
				Err(_) => {
					let color;
					self.pieces.push(match x.to_lowercase().as_str() {
						"r" => {
							Piece{
								piece: PieceType::Rook(true),
								color,
								position : row*8 + col
							}
						},// rook logic
						"b" => {
							Piece{
								piece: PieceType::Rook(true),
								color,
								position : row*8 + col
							}
						}, // bishop logic
						"n" => {
							
						},// knight logic
						"q" => {
							
						},// queen logic
						"k" => {
							
						}, // king logic
						"/" => {
							col = 0;
							row += 1;
						}// end of 
						_ => {panic!("Invalid char in FEN string : {}", x);}
					})
				}
			}
			
		}	
	}
	// gets the valid moves for a given position
	// IDK how to do this yet
	fn valid_moves(&self) -> Vec<Move>{
		let out:Vec<Move> = vec![];
		out
	}
	// makes a given move
	// removes taken pieces.
	fn make_move(&self, the_move : Move){

	}
	// unmakes a given move
	// replaces pieces that were captured if necessary.
	fn unmake_move(&self, the_move : Move){

	}
	// checks if a state is an end state
	// if it is then return the color of winner as well.
	fn end(&self) -> (bool, Option<Color>){
		(false, None)
	}
	// static evaluation of a given board state
	fn evaluation(&self) -> i32{
		0
	}
}