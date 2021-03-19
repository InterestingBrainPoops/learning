mod board;
fn main() {
    println!("Hello, world!");
    let pieces: Vec<board::Piece> = vec![];
    let mut test = board::Board {
        pieces,
        turn: board::Color::Black,
    };
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    test.from_fen(&fen);
    println!("{:#?}", test.pieces);
}
