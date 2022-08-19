use crate::reset::Reset;
use crate::tree::Tree;

impl Tree<Reset> {

    /// Generate the next child in a Reset Tree
    ///
    /// Returns Boolean indicating `true` if move options have not been exhausted
    /// and `false` if they have.
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
    pub fn add_next_move(&mut self) -> bool {
        // This will probably benefit from a freelist
        let mut reset = reset::new();
        let result = self.reset.generate_next_move(&mut reset);
        if !result {
            return false;
        }
        let mut child = tree::new(reset);
        self.add_child_last(child);
        true
    }

}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::utils;
    use crate::reset::Reset;

    #[test]
    fn generate_next_move() {
        let mut r = reset::new();
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        r.init_from_fen(fen.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0000000000000001,"b_current_piece");
        assert_eq!(r.move_id,10,"move_id");

        let mut r = reset::new();
        let fen2 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
        r.init_from_fen(fen2.to_string());
        r.initialize_move_generation();
        assert_eq!(r.b_current_piece,0x0001000000000000,"b_current_piece");
        assert_eq!(r.move_id,10,"move_id");
    }

}

