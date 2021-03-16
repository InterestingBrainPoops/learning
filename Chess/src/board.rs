// Chess board
pub struct Board{
    pieces : Vec<Piece>,
    turn : Color
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
    Pawn
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
    fn from_FEN(&self, fen:&String){

    }
    // gets the valid moves for a given position
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