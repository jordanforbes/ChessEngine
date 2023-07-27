pub mod Pieces{
  pub mod pawn;
  pub mod bishop;
}

pub use Pieces::pawn::Pawn;
pub use Pieces::bishop::Bishop;


pub struct Chessboard {
    board: [[Piece; 8]; 8],
}

impl Chessboard {
    // Implement any necessary methods for the chessboard
    // For example, you may have a method to place pieces on the board

        // Method to place the bishop piece on the board at position (x, y)
    pub fn set_up_bishop(&mut self, x: usize, y: usize) {
        self.board[x][y] = Piece::Bishop(Bishop::new()); // Create a new Bishop instance if needed
    }
}
