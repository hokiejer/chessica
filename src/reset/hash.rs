use crate::reset::Reset;
use crate::reset::PieceType;

impl Reset {
    /// Return a unique hash value for this child reset
    /// 
    /// # Examples
    ///
    /// ```
    /// # use chessica::reset::Reset;
    /// let mut r = chessica::reset::new();
    /// let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// r.init_from_fen(fen);
    /// assert_eq!(r.child_hash(),0xffff00000000ffff0000000000000000);
    /// ```
    pub fn child_hash(&mut self) -> u128 {
        let mut child_hash: u128 = 0x0;
        child_hash |= (self.b_all as u128) << 64;
        child_hash |= (self.bi_from as u128) << 56;
        child_hash |= (self.bi_to as u128) << 48;
        child_hash |= (self.promotion_piece as u128) << 40;
        child_hash
    }

}

#[cfg(test)]
mod tests {
    use crate::reset;
    use crate::reset::PieceType;
    #[test]
    fn child_hash_test() {
        let mut r = reset::new();
        r.b_all = 0x1234567812345678;
        r.bi_from = 0x44;
        r.bi_to = 0x22;
        r.promotion_piece = PieceType::Rook;
        println!("child_hash == {:x}",r.child_hash());
        assert_eq!(r.child_hash(),0x12345678123456784422040000000000);
    }

}