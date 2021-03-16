use std::{cmp::max, fmt, time::Instant};
use std::io;
#[derive(Debug, Clone, PartialEq)]
enum Square{
	Empty,
	X,
	O
}
#[derive(Debug, PartialEq, Clone)]
struct State {
	board : [[Square;3];3],
}
impl fmt::Display for State{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f,"{} | {} | {} \n {} | {} | {} \n {} | {} | {} ", self.board[0][0], self.board[0][1], self.board[0][2], self.board[1][0], self.board[1][1], self.board[1][2], self.board[2][0], self.board[2][1], self.board[2][2])
	}
}
impl Default for Square{
	fn default() -> Self {
		Square::Empty
	}
}
impl fmt::Display for Square{
	fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
		match self {
			Square::Empty => write!(f," - "),
			Square::X => write!(f," X "),
			Square::O => write!(f," O ")
		}
	}
}
// TODO: 
// Implement the skeleton functions, along with the Move struct.
impl State{
	fn is_end_state(&self) -> bool{
		let mut total = 0;
		for e in self.board.iter(){
			for x in e.iter(){
				match x{
					Square::Empty => total += 1,
					Square::X => total += 0,
					Square::O => total += 0,
				}
			}
		}
		for e in 0..self.board.len(){
			if (self.board[e][0] == self.board[e][1]) && (self.board[e][1]== self.board[e][2]) && (self.board[e][1] != Square::Empty){
				return true // row filled all with same square
			}
			if (self.board[0][e] == self.board[1][e]) && (self.board[1][e]== self.board[2][e]) && (self.board[1][e] != Square::Empty){
				return true // column all filled with same square
			}
		}
		if (self.board[0][0] == self.board[1][1]) && (self.board[1][1]== self.board[2][2]){// \ diagonal
			match self.board[0][0]{
				Square::Empty => {},
				_ => return true
			}
		}
		if (self.board[0][2] == self.board[1][1]) && (self.board[1][1]== self.board[2][0]){ // / diagonal
			match self.board[1][1]{
				Square::Empty => {},
				_ => return true
			}
		}
		if total == 0i32 {
			true // board is completely full and noone has won.
		}else{
			false // default case.
		}
	}
	fn static_eval(&self) -> i32{
		// bigger is better for O, lower is better for X.
		let mut total = 0;
		for e in self.board.iter(){
			for x in e.iter(){
				match x{
					Square::Empty => total += 1,
					Square::X => total += 0,
					Square::O => total += 0,
				}
			}
		}
		for e in 0..self.board.len(){
			if (self.board[e][0] == self.board[e][1]) && (self.board[e][1]== self.board[e][2]){
				// row filled all with same square
				match self.board[e][0]{
					Square::X => return i32::MIN,
					Square::O => return i32::MAX,
					_ => ()
				}
			}
			if (self.board[0][e] == self.board[1][e]) && (self.board[1][e]== self.board[2][e]){
				// column all filled with same square
				match self.board[0][e]{
					Square::X => {println!("{}", self); return i32::MIN},
					Square::O => return i32::MAX,
					_ => ()
				}
			}
		}
		if (self.board[0][0] == self.board[1][1]) && (self.board[1][1]== self.board[2][2]){
			match self.board[0][0]{
				Square::X => return i32::MIN,
				Square::O => return i32::MAX,
				_ => ()
			}
		}
		if (self.board[0][2] == self.board[1][1]) && (self.board[1][1]== self.board[2][0]){
			match self.board[1][1]{
				Square::X => return i32::MIN,
				Square::O => return i32::MAX,
				_ => ()
			}
		}
		if total == 0i32 {
			// println!("{}", self);
			0 // board is completely full and noone has won
			
		}else{
			0 // default case.
		}
	}
	fn possible_moves(&self) -> Vec<Move>{
		// first find whose turn it is
		// x goes first ( didnt know that )
		let mut out:Vec<Move> = vec![];
		let mut o_squares:i32 = 0;
		let mut x_squares = 0;
		let turn;
		for e in self.board.iter(){
			for x in e.iter(){
				match x{
					Square::Empty => (),
					Square::X => x_squares += 1,
					Square::O => o_squares += 1,
				}
			}
		}
		if x_squares > o_squares {
			turn = Square::O;
		}else{
			turn = Square::X;
		}
		// then get all empty squares coordinates
		// create a bunch of moves while doing so and adding them to a vector.
		for x in 0..self.board.len(){
			for y  in 0..self.board[x].len(){
				match self.board[x][y]{
					Square::Empty => out.push(Move{pos:(y as u8,x as u8), player:turn.clone()}),
					_ => ()
				}
			}
		}
		// return vector of moves.
		return out;
	}
	fn make_move(&mut self, movetomake : &Move){
		self.board[usize::from(movetomake.pos.1)][usize::from(movetomake.pos.0)] = movetomake.player.clone();
	}
	fn unmake_move(&mut self, movetounmake : &Move){
		self.board[usize::from(movetounmake.pos.1)][usize::from(movetounmake.pos.0)] = Square::Empty;
	}
}
#[derive(Debug, PartialEq)]
struct Move{
	player : Square, 
	pos : (u8,u8)
}
fn minimax(position: &mut State, depth : u32, mut alpha : i32, mut beta : i32, maximizing_player: bool, posis : &mut Vec<State>) -> i32{
	if !posis.iter().any(|i| *i==*position) {
		// if the current position is not yet visited, then add it to the vector:
		posis.push(position.clone());
	}
	if depth == 0 || position.is_end_state(){
		// println!("here {}", alpha);
		// println!("{}", position);
		return position.static_eval()
	}
	// println!("{} {}",depth == 0, alpha);
	if maximizing_player {
		let mut max_eval = i32::MIN;
		let mut eval:i32;
		for e in position.possible_moves().iter(){
			// println!("{}", position);
			position.make_move(e);
			eval = minimax(position, depth - 1, alpha.clone(), beta.clone(), false, posis);
			
			position.unmake_move(e);
			// println!("{}", position);
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
			eval = minimax(position, depth - 1, alpha.clone(), beta.clone(), true, posis);
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
	let mut possiblepositions:Vec<State> = vec![];

	let board:[[Square;3];3] = Default::default();
	let mut pos = State{board:board};
	let now = Instant::now();
	let e = minimax(&mut pos, 9, i32::MIN, i32::MAX, true, &mut possiblepositions);
	let new_now = Instant::now();
	println!("{:?}", new_now.duration_since(now));
	println!("{:?}", pos.possible_moves());
	println!("{:?} {:?}", e, possiblepositions.len());
	while !pos.is_end_state(){
		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");
		
		
		println!("{}",guess);
		if guess == "break" {
			break;
		}
	}
}