use crate::reset::Reset;
use crate::utils::convert_bitstring_to_square;

/// Prints a Reset
/// 
/// # Examples
///
/// ```
/// # use chessica::reset::Reset;
/// let mut r = chessica::reset::new();
/// r.print();
/// ```
impl Reset {
    pub fn print(&mut self) -> String {
        let piece_text = if self.b_to & self.b_pawns != 0 {
            if self.white_to_move() {
                "p"
            } else {
                "P"
            }
        } else if self.b_to & self.b_knights != 0 {
            if self.white_to_move() {
                "n"
            } else {
                "N"
            }
        } else if self.b_to & self.b_bishops != 0 {
            if self.white_to_move() {
                "b"
            } else {
                "B"
            }
        } else if self.b_to & self.b_rooks != 0 {
            if self.white_to_move() {
                "r"
            } else {
                "R"
            }
        } else if self.b_to & self.b_kings != 0 {
            if self.white_to_move() {
                "k"
            } else {
                "K"
            }
        } else if self.white_to_move() {
            "q"
        } else {
            "Q"
        };
        let from_text = convert_bitstring_to_square(self.b_from);
        let to_text = convert_bitstring_to_square(self.b_to);
        println!("{}:{}-{} => {}",piece_text,from_text,to_text,self.to_fen());
        self.to_fen()
    }
}


