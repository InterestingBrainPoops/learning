
// Chess board 
// each one has a list of 3 u64, each a bitboard. 
// [black, white, both]
#[derive(Debug)]
pub struct Board {
    zobrist : u64, // zobrist hash, starts at 0.
    bishop : [u64;3], // Bishop positions
    king : [u64;3], // King positions
    queen : [u64;3], // Queen positions 
    knight : [u64;3], // Knight positions
    rook : [u64;3], // Rook positions
    pawn : [u64;3],
    pub turn: Color,
}
// Piece implementation/ representation
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece: PieceType,
    color: Color,
    position: i8,
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
    move_by: i8,            // how much its moving by
    capture: Option<Piece>, // if capture put the captured piece here
    castle: Option<Piece>,  // if castling, put the rook here
    promote: Option<Piece>, // if promoted, to what piece.
}

impl Board {
    // constructor
    pub fn new() -> Board{
        Board{
            zobrist : 0,
            bishop : [0;3],
            king : [0;3],
            knight : [0;3],
            queen : [0;3],
            rook : [0;3],
            pawn : [0;3],
            turn : Color::White,
        }
    }
    // sets the board to the fen string
    pub fn from_fen(&mut self, fen: &String) {
    
    }
    // gets the valid moves for a given position
    // IDK how to do this yet
    fn valid_moves(&self) -> Vec<Move> {
        let out: Vec<Move> = vec![];
        out
    }
    // makes a given move
    // removes taken pieces.
    fn make_move(&self, the_move: Move) {
    }
    // unmakes a given move
    // replaces pieces that were captured if necessary.
    fn unmake_move(&self, the_move: Move) {

    }
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
