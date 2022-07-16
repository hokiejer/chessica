use crate::reset::Reset;

impl Reset {

    /// Is it white's move?
    ///
    /// # Examples
    /// ```
    /// let mut r = chessica::reset::new();
    /// let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    /// r.init_from_fen(fen.to_string());
    /// assert!(r.white_to_move());
    /// ```
    pub fn white_to_move(&self) -> bool {
        self.to_move == 0
    }
}

#[cfg(test)]
mod tests { 
    use crate::reset;

    #[test]
    fn helpers_white_to_move() {
        let mut r = reset::new();
        r.to_move = 0;
        assert!(r.white_to_move());
        r.to_move = 1;
        assert!(!r.white_to_move());
    }
}
