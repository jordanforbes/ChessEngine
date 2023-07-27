pub struct Bishop{

}

impl Bishop {
  // constructor method to create a new bishop instance if needed
  pub fn new() -> Self {
    Bishop
  }

  pub fn generate_bishop_moves(&self, x: usize, y: usize, chessboard: &Chessboard) -> Vec<(usize, usize)>{
    let mut moves = Vec::new();

    // define 4 diagonal directions (dx, dy)
    let directions = [(1,1), (-1,1), (1,-1), (-1,-1)];

    // iterate through each direction and calculate possible moves
    for (dx, dy) in &directions {
      let mut current_x = x as isize + dx;
      let mut current_y = y as isize + dy;

      // continue moving until we reach the end or an obstruction
      while current_x >= 0 && current_x <8 && current_y >= 0 && current_y <8 {
        let to_x = current_x as usize;
        let to_y = current_y as usize;

        // is destination square empty or occupied by enemy?
        if chessboard.board[to_X][to_y] == Pieces::Empty {
          moves.push((to_x, to_y));
        }
        else if /*is destination occupied by enemy?*/ {
          moves.push((to_x, to_y));
          break; //stop moving in this direction because opponent piece
        }else{
          break; // stop moving because friendly piece
        }

        // move to next square in diagonal direction
        current_x += dx;
        current_y += dy;
      }
    }

    moves
  }

  // is the bishop move valid?
  fn is_valid_move_bishop(&self, from_x: usize, from_y: usize, to_x: usize, to_y: usize, chessboard:&Chessboard) -> bool {
    // check if out of bounds
    if to_x >= 8 || to_y >= 8{
      return false;
    }

    // calculate horizontal and vertical distance
    let dx = (to_x as isize - from_x as isize).abs();
    let dy = (to_y as isize - from_y as isize).abs();

    //is move diagonal?
    if dx != dy {
      return false;
    }

    // is destination square empty?
    if chessboard.board[to_x][to_y] == Piece::Empty{
      return true;
    }else if{
      return true;
    }

    // If none of the conditions above were satisfied, the move is invalid
    false
  }

}
