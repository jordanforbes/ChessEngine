extern crate chess_engine;
use chess_engine::Pawn;

enum Piece {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

struct Chessboard{
    board: [[Piece: 8]; 8]
}

// initialize chessboard
fn Chessboard{
    fn new()-> Self {
        // Code to set up the starting position
        // Initialize the board with the correct pieces at their starting positions
        // Set player turn, etc.
    }
}


fn evaluate_position(chessboard: &Chessboard)-> i32{
    // evaluate position and return score
}

fn minmax{
    chessboard: &Chessboard,
    depth: usize,
    alpha: i32,
    beta: i32,
    maximizing_player: bool,
}-> i32{
    // code to implement min max algorighthm
}


impl Chessboard{
    let pawn = Pawn;

    fn generate_all_moves(&self) -> Vec<(usize, usize, usize, usize)>{
        // Code to generate moves for all pieces of the current player
        // Iterate through the board and call specific move generation functions for each piece
    }
}

// game loop
fn main() {
    let mut chessboard = Chessboard::new();

    loop {

    }
}
