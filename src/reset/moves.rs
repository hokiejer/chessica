use crate::reset::Reset;
use crate::bitops;
use crate::reset::r#const::B_FOUR_CORNERS;
use crate::reset::r#const::B_LOWER_RIGHT_CORNER;
use crate::reset::r#const::B_LOWER_LEFT_CORNER;
use crate::reset::r#const::B_UPPER_RIGHT_CORNER;
use crate::reset::r#const::B_UPPER_LEFT_CORNER;

impl Reset {

    /// Prepare a Reset to generate moves
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// ```
    pub fn initialize_move_generation(&mut self) {
        if self.white_to_move() {
            self.b_current_piece = bitops::lowest_bit(self.b_white);
        } else {
            self.b_current_piece = bitops::lowest_bit(self.b_black);
        }
        self.current_piece = bitops::get_bit_number(self.b_current_piece);
        self.move_id = 10;	//Prime the first move
    }

    /// Consider the next moveable piece
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// ```
    pub fn consider_next_moveable_piece(&mut self) {
        if self.white_to_move() {
            self.b_current_piece = bitops::next_lowest_bit(self.b_white, self.b_current_piece);
        } else {
            self.b_current_piece = bitops::next_lowest_bit(self.b_black, self.b_current_piece);
        }
    }


    /// Generate the next move for a Reset
    ///
    /// Returns Boolean indicating `true` if move options have not been exhausted
    /// and `false` if they have.  Expects the child reset to already be initialized 
    /// from the parent.
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let mut child = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// r.initialize_move_generation();
    /// r.generate_next_move(&mut child);
    /// ```
    pub fn generate_next_move(&mut self, child: &mut Reset) -> bool {
        while self.b_current_piece != 0 {
            if self.b_current_piece & self.b_pawns != 0 { // Pawn
                if self.generate_next_pawn_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_knights != 0 { // Knight
                if self.generate_next_knight_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_bishops != 0 { // Bishop
                if self.generate_next_bishop_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_rooks != 0 { // Rook
                if self.generate_next_rook_move(child) {
                    break;
                }
            } else if self.b_current_piece & self.b_queens != 0 { // Queen
                if self.generate_next_queen_move(child) {
                    break;
                }
            } else { // King
                if self.generate_next_king_move(child) {
                    break;
                }
            }

            // do stuff
            self.consider_next_moveable_piece();
        }
        self.b_current_piece > 0
    }


    /// Adds a move to the specified child reset if valid
    ///
    pub fn add_move_if_valid(&mut self, child: &mut Reset, b_destination: u64) -> bool {

        child.b_from = self.b_current_piece;
        child.b_to = b_destination;

        if child.b_to & child.b_all != 0 { // Capture
            child.capture_processing();
        }
        // Pick up the piece
        child.b_all &= child.b_from;
        child.b_white &= !child.b_from;
        child.b_black &= !child.b_from;
        if child.b_from & child.b_pawns != 0 {
            child.b_pawns &= !child.b_from;
            child.halfmove_clock = 0; // Resets on pawn move
        } else if child.b_from & child.b_knights != 0 {
            child.b_knights &= !child.b_from;
        } else if child.b_from & child.b_bishops != 0 {
            child.b_bishops &= !child.b_from;
        } else if child.b_from & child.b_rooks != 0 {
            child.b_rooks &= !child.b_from;
            if child.b_from & B_FOUR_CORNERS != 0 {
                if child.b_to & B_LOWER_RIGHT_CORNER != 0 {
                    child.white_castle_k = 0;
                } else if child.b_to & B_LOWER_LEFT_CORNER != 0 {
                    child.white_castle_q = 0;
                } else if child.b_to & B_UPPER_RIGHT_CORNER != 0 {
                    child.black_castle_k = 0;
                } else { // B_UPPER_RIGHT_CORNER
                    child.black_castle_q = 0;
                }
            }
        } else if child.b_from & child.b_queens != 0 {
            child.b_queens &= !child.b_from;
        } else {
            child.b_kings &= !child.b_from;
            child.white_castle_k = 0;
            child.white_castle_q = 0;
            child.black_castle_k = 0;
            child.black_castle_q = 0;
        }


        // Put down the piece
        
        if self.white_to_move() {
        } else {
        }
        true
    }
}
//int Reset::AddNextWhiteMove(Reset *MyChild, unsigned long long int *PieceBeingMoved)
//{
//  MyChild->bFrom = bCurrentPiece;
//  MyChild->From = CurrentPiece;
//  MyChild->bAll &= ~MyChild->bFrom;
//  MyChild->bWhite &= ~MyChild->bFrom;
//  *PieceBeingMoved &= ~MyChild->bFrom;
//  MyChild->bTo = bMoveData;
//  MyChild->To = MoveData;
//  if (MyChild->bTo & MyChild->bAll)
//  {
//    MyChild->CaptureProcessing(MyChild->bTo);
//    MyChild->Capture = 1;
//    MyChild->MovesSinceCapture = 0; //Reset on capture
//  }
//  else
//  {
//    if (MyChild->bTo & MyChild->bPawns)
//      MyChild->MovesSinceCapture = 0; //Reset on pawn move
//    else
//      MyChild->MovesSinceCapture = MovesSinceCapture + 1;
//  }
//  MyChild->bAll |= MyChild->bTo;
//  MyChild->bWhite |= MyChild->bTo;
//  *PieceBeingMoved |= MyChild->bTo;
//  if ((MyChild->DidWhiteJustMoveIntoCheck()) ||
//      (MyChild->MovesSinceCapture >= 50))
//  {
//    InitMyChild(MyChild);
//    return(FALSE);
//  }
//  if (MyChild->DidWhiteMoveCauseBlackCheck())
//  {
//    MyChild->BlackInCheck = ON;
//  }
//  MyChild->HashValue = (int) (MyChild->bAll % LARGE_PRIME);
//  return(TRUE);
//}

#[cfg(test)]
mod tests {
    use crate::reset;

    #[test]
    fn move_init_move_generation() {
        let mut r = reset::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0000000000000001,"b_current_piece");
        assert_eq!(r.current_piece,1,"current_piece");
        assert_eq!(r.move_id,10,"move_id");

        let mut r = reset::new();
        let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        r.init_from_fen(fen2.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0001000000000000,"b_current_piece");
        assert_eq!(r.current_piece,49,"current_piece");
        assert_eq!(r.move_id,10,"move_id");
    }

    #[test]
    fn move_consider_next_moveable_piece() {
        let mut r = reset::new();
        let fen = "4k2r/8/8/8/8/8/8/R3K3 w Qk - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0000000000000008);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000080);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000000);

        let mut r = reset::new();
        let fen = "4k2r/8/8/8/8/8/8/R3K3 b Qk - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0100000000000000);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0800000000000000);
        r.consider_next_moveable_piece();
        assert_eq!(r.b_current_piece,0x0000000000000000);
    }
}
