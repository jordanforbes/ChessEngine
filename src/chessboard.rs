impl Chessboard{
    fn generate_pawn_moves(&self, x:usize, y:usize) -> Vec<(usize, usize)>{
        // generate legal moves for pawns at x and y positions
        let mut moves = Vec::new();

        // generate potential moves for the pawn
        let destinations = [
            (from_x + 1, from_y)        //move 1 square forward
            (from_x + 2, from_y)        // move 2 squares forward on the first move
            (from_x + 1, from_y + 1)    // capture diagonally right
            (from_x + 1, from_y - 1)    // capture diagonally left
        ]

        // filter valid moves using move validation function
        for &(to_x, to_y) in &destinations{
            if self.is_valid_move_pawn(from_X, from_y, to_x, to_y){
                moves.push((to_x, to_y));
            }
        }
        moves
    }
    fn generate_all_moves(&self) -> Vec<(usize, usize, usize, usize)>{
        // Code to generate moves for all pieces of the current player
        // Iterate through the board and call specific move generation functions for each piece
    }
    fn is_valid_move_pawn(&self, from_x:usize, from_y:usize, to_x:usize, to_y:usize) -> bool{
        // check if out of bounds
        if to_x >=8|| to_y >=8{
            return false;
        }

        // implement pawn movement rules
        let piece = self.board[from_x][from_y];

        match piece {
            // logic for pawn piece
            Piece::Pawn =>{
                // is it the first move?
                let is_first_move = from_x ==1 || from_y == 6;

                // is the destination empty?
                if to_x == from_x+1 && to_y == from_y && self.board[to_x][to_y] == Piece::Empty{
                    return true;
                }

                // can pawn move two squares forward on first move?
                if is_first_move && to_x == from_x + 2 && self.board[to_x][to_y] == Piece::Empty{
                    return true;
                }

                // can pawn capture diagonally?
                if (to_x == from_x +1 && (to_y == from_y + 1 || to_y == from_y -1)) && self.board[to_x][to_y] != Piece::Empty
                {
                    return true;
                }

            }
        }

    }
}
