use std::{cmp::max, time::Instant};
#[derive(Debug, Clone, PartialEq)]
enum Square{
	Empty,
	X,
	O
}
#[derive(Debug)]
struct State {
	Board : [[Square;3];3],
}
impl Default for Square{
	fn default() -> Self {
		Square::Empty
	}
}
// TODO: 
// Implement the skeleton functions, along with the Move struct.
impl State{
	fn is_end_state(&self) -> bool{
		let mut total = 0;
		for e in self.Board.iter(){
			for x in e.iter(){
				match x{
					Square::Empty => total += 1,
					Square::X => total += 0,
					Square::O => total += 0,
				}
			}
		}
		if total == 0i32 {
			true // board is completely full
		}else{
			for e in 0..self.Board.len(){
				if (self.Board[e][0] == self.Board[e][1]) && (self.Board[e][1]== self.Board[e][2]) && (self.Board[e][1] != Square::Empty){
					return true // row filled all with same square
				}
				if (self.Board[0][e] == self.Board[1][e]) && (self.Board[1][e]== self.Board[2][e]) && (self.Board[1][e] != Square::Empty){
					return true // column all filled with same square
				}
			}
			false // default case.
		}
	}
	fn static_eval(&self) -> i32{
		// bigger is better for O, lower is better for X.
		let mut total = 0;
		for e in self.Board.iter(){
			for x in e.iter(){
				match x{
					Square::Empty => total += 1,
					Square::X => total += 0,
					Square::O => total += 0,
				}
			}
		}
		if total == 0i32 {
			0 // board is completely full
		}else{
			for e in 0..self.Board.len(){
				if (self.Board[e][0] == self.Board[e][1]) && (self.Board[e][1]== self.Board[e][2]){
					// row filled all with same square
					match self.Board[e][0]{
						Square::X => return i32::MIN,
						Square::O => return i32::MAX,
						_ => return 0
					}
				}
				if (self.Board[0][e] == self.Board[1][e]) && (self.Board[1][e]== self.Board[2][e]){
					// column all filled with same square
					match self.Board[0][e]{
						Square::X => return i32::MIN,
						Square::O => return i32::MAX,
						_ => return 0
					}
				}
			}
			0 // default case.
		}
	}
	fn possible_moves(&self) -> Vec<Move>{
		// first find whose turn it is
		// x goes first ( didnt know that )
		let mut out:Vec<Move> = vec![];
		let mut o_squares:i32 = 0;
		let mut x_squares = 0;
		let mut turn;
		for e in self.Board.iter(){
			for x in e.iter(){
				match x{
					Square::Empty => continue,
					Square::X => x_squares += 1,
					Square::O => o_squares += 1,
				}
			}
		}
		if o_squares > x_squares {
			turn = Square::O;
		}else{
			turn = Square::X;
		}
		// then get all empty squares coordinates
		// create a bunch of moves while doing so and adding them to a vector.
		for x in 0..self.Board.len(){
			for y  in 0..self.Board[x].len(){
				match self.Board[x][y]{
					Square::Empty => out.push(Move{pos:(y as u8,x as u8), player:turn.clone()}),
					_ => ()
				}
			}
		}
		// return vector of moves.
		return out;
	}
	fn make_move(&mut self, movetomake : &Move){
		self.Board[usize::from(movetomake.pos.1)][usize::from(movetomake.pos.0)] = movetomake.player.clone();
	}
	fn unmake_move(&mut self, movetounmake : &Move){
		self.Board[usize::from(movetounmake.pos.1)][usize::from(movetounmake.pos.0)] = Square::Empty;
	}
}
#[derive(Debug)]
struct Move{
	player : Square, 
	pos : (u8,u8)
}
fn minimax(position: &mut State, depth : u32, mut alpha : i32, mut beta : i32, maximizing_player: bool) -> i32{
	if depth == 0 || position.is_end_state(){
		// println!("here {}", alpha);
		return position.static_eval()
	}
	// println!("{} {}",depth == 0, alpha);
	if maximizing_player {
		let mut max_eval = i32::MIN;
		let mut eval:i32;
		for e in position.possible_moves().iter(){
			position.make_move(e);
			eval = minimax(position, depth - 1, alpha.clone(), beta.clone(), false);
			position.unmake_move(e);
			max_eval = max(max_eval, eval);
			alpha = max(alpha, eval);
			if beta <= alpha {
				break
			}
		}
		max_eval
	}else{
		let mut min_eval = i32::MAX;
		let mut eval:i32;
		for e in position.possible_moves().iter(){
			position.make_move(e);
			eval = minimax(position, depth - 1, alpha.clone(), beta.clone(), false);
			// println!("{}", eval);
			position.unmake_move(e);
			min_eval = min_eval.min(eval);
			beta = beta.min(eval);
			if beta <= alpha {
				break
			}
		}
		min_eval
	}

}
fn main() {
	let board:[[Square;3];3] = Default::default();
	let mut pos = State{Board:board};

	let now = Instant::now();
	let e = minimax(&mut pos, 9, i32::MIN, i32::MAX, true);
	let new_now = Instant::now();
	println!("{:?}", new_now.duration_since(now));
	println!("{:?}", pos.possible_moves());
	println!("{:?}", e);
}